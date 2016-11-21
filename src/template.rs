use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use compiler;
use parser;
use parser::Node;
use parser::Node::{Value, Static, Unescaped, Section, Part};
use Data;
use Data::{Strng, Bool, Integer, Float, Vector, Hash, Lambda};
use build::HashBuilder;
use std::collections::HashMap;

use RustacheResult;
use RustacheError::TemplateErrorType;
use self::TemplateError::*;

pub struct Template {
    partials_path: String,
}

#[derive(Debug)]
pub enum TemplateError {
    StreamWriteError(String),
    FileReadError(String),
    UnexpectedDataType(String),
    UnexpectedNodeType(String),
}

impl Template {
    pub fn new() -> Template {
        Template { partials_path: String::new() }
    }

    // utility method to write out rendered template with error handling
    fn write_to_stream<W: Write>(&self,
                                 writer: &mut W,
                                 data: &str,
                                 errstr: &str)
                                 -> RustacheResult<()> {
        let mut rv: RustacheResult<()> = Ok(());
        let status = writer.write_fmt(format_args!("{}", &data[..]));
        if let Err(err) = status {
            let msg = format!("{}: {}", err, errstr);
            rv = Err(TemplateErrorType(StreamWriteError(msg)));
        }

        rv
    }

    // method to escape HTML for default value tags
    fn escape_html(&self, input: &str) -> Box<String> {
        let mut rv = Box::new(String::new());
        for c in input.chars() {
            match c {
                '<' => {
                    rv.push_str("&lt;");
                }
                '>' => {
                    rv.push_str("&gt;");
                }
                '&' => {
                    rv.push_str("&amp;");
                }
                '"' => {
                    rv.push_str("&quot;");
                }
                _ => {
                    rv.push(c);
                }
            }
        }
        rv
    }

    // key:       the key we're looking for
    // sections:  an array of the nested sections to look through, e.g. [e, d, c, b, a]
    // datastore: the hash of the data to search for key in
    //
    // TODO: handle vector data for real, change to not build vector, but
    // iterate the same way until data is found
    //
    fn look_up_section_data<'a, 'b>(&self,
                                    key: &str,
                                    sections: &Vec<String>,
                                    datastore: &'b HashMap<String, Data<'a>>)
                                    -> Option<&'b Data<'a>> {
        let mut rv = None;
        let mut hashes = Vec::new();
        let mut hash = datastore;


        // any kind of tag may be in a nested section, in which case it's data
        // may be in a context further up, so we have to have a way to search
        // up those contexts for a value for some key.
        //
        // so a template of {{#a}}{{#b}}{{#c}}{{value}}{{/c}}{{/b}}{{/a}}
        // and data of { a: { b: { "value": "foo", c: {}}}
        // we should be able to find "foo" even though it is not under "c"'s data
        //
        // to do this, we look, first through a nested path.  we take the hash
        // found for each section, starting with the most nested to the outside,
        // and push references their sub-hashes onto a vector.
        //
        // so with data of { a: { b: { "value": "foo", c: {"cdata": foo}}}
        // we end up with a vector: [{"cdata":"foo"},
        //                           {"value": "foo", "c": { "cdata": foo }},
        //                           { b: { "value": "foo", c: {"cdata": foo}}]
        for section in sections.iter() {
            match hash.get(section) {
                None => {}
                Some(data) => {
                    match *data {
                        Hash(ref h) => {
                            hashes.insert(0, h);
                            hash = h;
                        }
                        _ => {}
                    }
                }
            }
        }

        // data for nested sections may also be in the top level of data,
        // so not only do we have to check up the nested structure, we have
        // to check the top level for each section data
        //
        // so a template of {{#a}}{{#b}}{{#c}}{{value}}{{/c}}{{/b}}{{/a}}
        // and data of { a: {}, b: { "value", "foo"}, c{} }
        // we should be able to find the value "foo"
        //
        // after this, we do the same for the top level datastore.  we need to do it
        // in this order so we look through nested first.
        // so with data { a: {}, b: { "value", "foo"}, c{} }
        // we end up with the previous vector plus: [{}, { "value", "foo"}, {}]
        //
        for section in sections.iter() {
            match datastore.get(section) {
                None => {}
                Some(data) => {
                    match *data {
                        Hash(ref h) => {
                            hashes.insert(0, h);
                        }
                        Vector(_) => {
                            return Some(data);
                        }
                        _ => {}
                    }
                }
            }
        }

        // once we've assembled the vector of hashes to look through
        // we iterate through it looking for the data
        for hash in hashes.iter() {

            rv = hash.get(key);
            if rv.is_some() {
                break;
            }
        }

        // last but not least, check the top level if we didn't find anything
        if rv.is_none() {
            rv = datastore.get(key);
        }

        rv
    }

    fn handle_unescaped_lambda_interpolation<W: Write>(&mut self,
                                                       f: &mut FnMut(String) -> String,
                                                       data: &HashMap<String, Data>,
                                                       raw: String,
                                                       writer: &mut W)
                                                       -> RustacheResult<()> {
        let val = (*f)(raw);
        let mut tokens = compiler::create_tokens(&val[..]);
        let nodes = parser::parse_nodes(&mut tokens);

        self.render(writer, data, &nodes)
    }

    fn handle_escaped_lambda_interpolation<W: Write>(&mut self,
                                                     f: &mut FnMut(String) -> String,
                                                     data: &HashMap<String, Data>,
                                                     raw: String,
                                                     writer: &mut W)
                                                     -> RustacheResult<()> {
        let val = (*f)(raw);
        let value = self.escape_html(&val[..]);
        let mut tokens = compiler::create_tokens(&value[..]);
        let nodes = parser::parse_nodes(&mut tokens);

        self.render(writer, data, &nodes)
    }

    // data:      the data value for the tag/node we're handling
    // key:       the name of the tag we're handling, i.e. the key into the data hash
    // datastore: all the data for the template
    // writer:    the output stream to write rendered template to
    //
    // the Data enum, which is how we hold different types of data in one hash,
    // can be, well, several different types.  this method matches them all and
    // handles the data appropriately.
    //
    // TODO: really don't need to be handling Bool, Vector or Hash
    fn handle_unescaped_or_value_node<W: Write>(&mut self,
                                                node: &Node,
                                                data: &Data,
                                                key: String,
                                                datastore: &HashMap<String, Data>,
                                                writer: &mut W)
                                                -> RustacheResult<()> {
        let mut rv = Ok(());
        let mut tmp: String = String::new();
        match *data {
            // simple value-for-tag exchange, write out the string
            Strng(ref val) => {
                match *node {
                    Unescaped(_, _) => tmp = tmp + val,
                    Value(_, _) => tmp = *self.escape_html(&val[..]),
                    _ => return Err(TemplateErrorType(UnexpectedNodeType(format!("{:?}", node)))),
                }
                rv = self.write_to_stream(writer, &tmp, "render: unescaped node string fail");
            }
            // TODO: this one doesn't quite make sense.  i don't think we need it.
            Bool(ref val) => {
                match val {
                    &true => tmp.push_str("true"),
                    &false => tmp.push_str("false"),
                }
                rv = self.write_to_stream(writer, &tmp, "render: unescaped node bool");
            }
            // if the data is an integer, convert it to a string and write that
            Integer(ref val) => {
                tmp = tmp + &val.to_string();
                rv = self.write_to_stream(writer, &tmp, "render: unescaped node int");
            }
            // if the data is a float, convert it to a string and write that
            Float(ref val) => {
                tmp = tmp + &val.to_string();
                rv = self.write_to_stream(writer, &tmp, "render: unescaped node float");
            }
            // TODO: this one doesn't quite make sense.  i don't think we need it.
            Vector(ref list) => {
                for item in list.iter() {
                    rv = self.handle_unescaped_or_value_node(node,
                                                             item,
                                                             key.to_string(),
                                                             datastore,
                                                             writer);
                    match rv {
                        Ok(_) => {}
                        _ => {
                            return rv;
                        }
                    }
                }
            }
            // TODO: this one doesn't quite make sense.  i don't think we need it.
            Hash(ref hash) => {
                if hash.contains_key(&key) {
                    let ref tmp = hash[&key];
                    rv = self.handle_unescaped_or_value_node(node,
                                                             tmp,
                                                             key.to_string(),
                                                             datastore,
                                                             writer);
                    match rv {
                        Ok(_) => {}
                        _ => {
                            return rv;
                        }
                    }
                }
            }
            // if we have a lambda for the data, the return value of the
            // lambda is what we substitute for the tag
            Lambda(ref f) => {
                let raw = "".to_string();
                match *node {
                    Unescaped(_, _) => {
                        rv = self.handle_unescaped_lambda_interpolation(&mut *f.borrow_mut(),
                                                                        datastore,
                                                                        raw,
                                                                        writer)
                    }
                    Value(_, _) => {
                        rv = self.handle_escaped_lambda_interpolation(&mut *f.borrow_mut(),
                                                                      datastore,
                                                                      raw,
                                                                      writer)
                    }
                    _ => return Err(TemplateErrorType(UnexpectedNodeType(format!("{:?}", node)))),
                }
            }
        }

        rv
    }

    // nodes:     children of the inverted section tag
    // datastore: all the data for the template
    // writer:    the io stream to write the rendered template to
    //
    // inverted nodes only contain static text to render and are only rendered
    // if the data in the template data for the tag name is "falsy"
    //
    fn handle_inverted_node<W: Write>(&mut self,
                                      nodes: &Vec<Node>,
                                      datastore: &HashMap<String, Data>,
                                      writer: &mut W)
                                      -> RustacheResult<()> {
        println!("handle inverted node: nodes: {:?}, datastore: {:?}",
                 nodes,
                 datastore);
        let mut rv = Ok(());
        for node in nodes.iter() {
            match *node {
                Static(key) => {
                    rv =
                        self.write_to_stream(writer,
                                             &key.to_string(),
                                             "render: inverted node static");
                }
                // TODO: this one doesn't quite make sense.  i don't think we need it.
                Part(filename, _) => {
                    rv = self.handle_partial_file_node(filename, datastore, writer);
                }
                Section(ref key, ref children, ref inverted, _, _) => {
                    let tmp = key.to_string();
                    let truthy = if datastore.contains_key(&tmp) {
                        self.is_section_data_true(&datastore[&tmp])
                    } else {
                        false
                    };
                    match (truthy, *inverted) {
                        (true, true) => {}
                        (false, false) => {}
                        (true, false) => {
                            let ref val = datastore[&tmp];
                            let mut sections = vec![tmp.clone()];
                            rv = self.handle_section_node(children,
                                                          &tmp,
                                                          val,
                                                          datastore,
                                                          &mut sections,
                                                          writer);
                        }
                        (false, true) => {
                            rv = self.handle_inverted_node(children, datastore, writer);
                        }
                    }
                }
                _ => {}
            }
        }

        rv
    }

    // nodes:     the section's children
    // data:      data from section key from HashBuilder store
    // datastore: HashBuilder data
    // writer:    io stream
    fn handle_section_node<W: Write>(&mut self,
                                     nodes: &Vec<Node>,
                                     _: &str,
                                     data: &Data,
                                     datastore: &HashMap<String, Data>,
                                     sections: &mut Vec<String>,
                                     writer: &mut W)
                                     -> RustacheResult<()> {
        let mut rv = Ok(());
        // there's a special case if the section tag data was a lambda
        // if so, the lambda is used to generate the values for the tag inside the section
        match *data {
            Lambda(ref f) => {
                let raw = self.get_section_text(nodes);
                return self.handle_unescaped_lambda_interpolation(&mut *f.borrow_mut(),
                                                                  datastore,
                                                                  *raw,
                                                                  writer);
            }
            Vector(ref v) => {
                for d in v.iter() {
                    for node in nodes.iter() {
                        match *d {
                            Hash(ref h) => {
                                rv = self.handle_node(node, h, writer);
                            }
                            Strng(ref val) => {
                                return Err(TemplateErrorType(UnexpectedDataType(format!("{}",
                                                                                        val))))
                            }
                            Bool(ref val) => {
                                return Err(TemplateErrorType(UnexpectedDataType(format!("{}",
                                                                                        val))))
                            }
                            Integer(ref val) => {
                                return Err(TemplateErrorType(UnexpectedDataType(format!("{}",
                                                                                        val))))
                            }
                            Float(ref val) => {
                                return Err(TemplateErrorType(UnexpectedDataType(format!("{}",
                                                                                        val))))
                            }
                            Vector(ref val) => {
                                return Err(TemplateErrorType(UnexpectedDataType(format!("{:?}",
                                                                                        val))))
                            }
                            Lambda(_) => {
                                return Err(TemplateErrorType(UnexpectedDataType("lambda"
                                    .to_string())))
                            }
                        }
                    }
                }
                return rv;
            }
            _ => {}
        }

        // in a section tag, there are child tags to fill out,
        // we need to iterate through each one
        for node in nodes.iter() {
            match *node {
                // unescaped is simple, just look up the data in the
                // special way sections need to and handle the node
                Unescaped(key, _) => {
                    let tmpkey = key.to_string();
                    let tmpdata = self.look_up_section_data(&tmpkey, sections, datastore);
                    if tmpdata.is_some() {
                        rv = self.handle_unescaped_or_value_node(node,
                                                                 tmpdata.unwrap(),
                                                                 key.to_string(),
                                                                 datastore,
                                                                 writer);
                    }
                }
                // unescaped is simple, just look up the data in the
                // special way sections need to and handle the node
                Value(key, _) => {
                    let tmpkey = key.to_string();
                    let tmpdata = self.look_up_section_data(&tmpkey, sections, datastore);
                    if tmpdata.is_some() {
                        rv = self.handle_unescaped_or_value_node(node,
                                                                 tmpdata.unwrap(),
                                                                 key.to_string(),
                                                                 datastore,
                                                                 writer);
                    }
                }
                // most simple, just write the static data out, nothing to replace
                Static(key) => {
                    rv =
                        self.write_to_stream(writer,
                                             &key.to_string(),
                                             "render: section node static");
                }
                // sections are special and may be inverted
                Section(ref key, ref children, ref inverted, _, _) => {
                    match *inverted {
                        // A normal, not inverted tag is more complicated and may recurse
                        // we need to save what sections we have been in, so the data
                        // lookup can happen correctly.  Data lookup is special for sections.
                        false => {
                            let tmpkey = key.to_string();
                            sections.push(tmpkey.clone());
                            let tmpdata = self.look_up_section_data(&tmpkey, sections, datastore);
                            if tmpdata.is_some() {
                                rv = self.handle_section_node(children,
                                                              &tmpkey,
                                                              tmpdata.unwrap(),
                                                              datastore,
                                                              sections,
                                                              writer);
                            }
                        }
                        // inverted only has internal static text, so is easy to handle
                        true => {
                            rv = self.handle_inverted_node(children, datastore, writer);
                        }
                    }
                }
                // if it's a partial, we have a file to read in and render
                Part(path, _) => {
                    rv = self.handle_partial_file_node(path, datastore, writer);
                }
            }
        }

        rv
    }

    // section data is considered false in a few cases:
    // there is no data for the key in the data hashmap
    // the data is a bool with a value of false
    // the data is an empty vector
    fn is_section_data_true(&self, data: &Data) -> bool {
        match *data {
            // if the data is a bool, rv is just the bool value
            Bool(value) => value,
            Vector(ref vec) => !vec.is_empty(),
            _ => true,
        }
    }

    // children: a vector of nodes representing the template text
    //           found between the section tags
    //
    // in the case of values for a section being lambdas, we need to pass
    // the raw text of the inside of the section tags to the lambda.
    // we store the raw text of each tag in the tag enum itself,
    // so we iterate through the children of the section, pulling out
    // the raw text and creating a string of it to pass to the lambda.
    //
    fn get_section_text(&self, children: &Vec<Node>) -> Box<String> {
        let mut temp = Box::new(String::new());
        for child in children.iter() {
            match child {
                &Static(text) => temp.push_str(text),
                &Value(_, ref text) => temp.push_str(&text[..]),
                &Section(_, ref children, _, ref open, ref close) => {
                    let rv = self.get_section_text(children);
                    temp.push_str(&open[..]);
                    temp.push_str(&rv[..]);
                    temp.push_str(&close[..]);
                }
                &Unescaped(_, ref text) => temp.push_str(&text[..]),
                &Part(_, text) => temp.push_str(text),
            }
        }
        temp
    }

    // filename:  the filename of the partial template to include,
    //            a.k.a the value inside the tag
    // datastore: all the template data
    // writer:    the io stream to write the rendered template out to
    //
    // in the mustache spec, it says parials are rendered at runtime,
    // so we call render in this method.  datastore and writer are taken
    // in as parameters because we have to do this
    //
    // TODO: throw error if partials file doesn't exist, if file read fails
    //
    fn handle_partial_file_node<W: Write>(&mut self,
                                          filename: &str,
                                          datastore: &HashMap<String, Data>,
                                          writer: &mut W)
                                          -> RustacheResult<()> {
        let mut rv: RustacheResult<()> = Ok(());;
        let path = Path::new(&self.partials_path.clone()).join(filename);
        if fs::metadata(&path).is_ok() {

            let mut contents = String::new();
            let file = File::open(&path).and_then(|ref mut f| f.read_to_string(&mut contents));
            match file {
                Ok(_) => {
                    let mut tokens = compiler::create_tokens(&contents[..]);
                    let nodes = parser::parse_nodes(&mut tokens);

                    rv = self.render(writer, datastore, &nodes);
                }
                Err(err) => {
                    let msg = format!("{}: {}", err, filename);
                    rv = Err(TemplateErrorType(FileReadError(msg)));
                }
            }
        } // if the file is not found, it's supposed to fail silently

        rv
    }

    fn handle_node<W: Write>(&mut self,
                             node: &Node,
                             datastore: &HashMap<String, Data>,
                             writer: &mut W)
                             -> RustacheResult<()> {
        let mut rv = Ok(());

        match *node {
            Unescaped(key, _) => {
                let tmp = key.to_string();
                if datastore.contains_key(&tmp) {
                    let ref val = datastore[&tmp];
                    rv = self.handle_unescaped_or_value_node(node,
                                                             val,
                                                             "".to_string(),
                                                             datastore,
                                                             writer);
                }
            }
            // value nodes contain tags who's data gets HTML escaped
            // when it gets written out
            Value(key, _) => {
                let tmp = key.to_string();
                if datastore.contains_key(&tmp) {
                    let ref val = datastore[&tmp];
                    rv = self.handle_unescaped_or_value_node(node,
                                                             val,
                                                             "".to_string(),
                                                             datastore,
                                                             writer);
                }
            }
            // static nodes are the test in the template that doesn't get modified,
            // just gets written out character for character
            Static(key) => {
                rv = self.write_to_stream(writer, &key.to_string(), "render: static");
            }
            // sections come in two kinds, normal and inverted
            //
            // inverted are if the tag data is not there, the Static between it
            // and it's closing tag gets written out, otherwise the text is thrown out
            //
            // normal section tags enclose a bit of html that will get repeated
            // for each element found in it's data
            Section(ref key, ref children, ref inverted, _, _) => {
                let tmp = key.to_string();
                let truthy = if datastore.contains_key(&tmp) {
                    self.is_section_data_true(&datastore[&tmp])
                } else {
                    false
                };
                match (truthy, *inverted) {
                    (true, true) | (false, false) => {}
                    (true, false) => {
                        let ref val = datastore[&tmp];
                        let mut sections = vec![tmp.clone()];
                        rv = self.handle_section_node(children,
                                                      &tmp,
                                                      val,
                                                      datastore,
                                                      &mut sections,
                                                      writer);
                    }
                    (false, true) => {
                        rv = self.handle_inverted_node(children, datastore, writer);
                    }
                }
            }
            // partials include external template files and compile and process them
            // at runtime, inserting them into the document at the point the tag is found
            Part(name, _) => {
                rv = self.handle_partial_file_node(name, datastore, writer);
            }
        }

        rv
    }
    // writer: an io::stream to write the rendered template out to
    // data:   the internal HashBuilder data store
    // parser: the parser object that has the parsed nodes, see src/parse.js
    pub fn render<W: Write>(&mut self,
                            writer: &mut W,
                            data: &HashMap<String, Data>,
                            nodes: &Vec<Node>)
                            -> RustacheResult<()> {
        let mut rv = Ok(());

        // nodes are what the template file is parsed into
        // we have to iterate through each one and handle it as
        // the kind of node it is
        for node in nodes.iter() {
            rv = self.handle_node(node, data, writer);
            if let Err(_) = rv {
                return rv;
            }

        }

        rv
    }

    // main entry point to Template
    pub fn render_data<W: Write>(&mut self,
                                 writer: &mut W,
                                 datastore: &HashBuilder,
                                 nodes: &Vec<Node>)
                                 -> RustacheResult<()> {
        // we need to hang on to the partials path internally,
        // if there is one, for class methods to use.
        self.partials_path.truncate(0);
        self.partials_path.push_str(datastore.partials_path);

        self.render(writer, &datastore.data, nodes)
    }
}


#[cfg(test)]
mod template_tests {
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, Cursor, Read, Seek, SeekFrom};
    use std::str;

    use parser;
    use parser::Node;
    use parser::Node::{Value, Static, Unescaped, Section, Part};
    use compiler;
    use template::Template;
    use build::{HashBuilder, VecBuilder};
    use Data::Strng;

    #[test]
    fn test_look_up_section_data() {
        let hb = HashBuilder::new()
            .insert("a",
                    HashBuilder::new()
                        .insert("b",
                                HashBuilder::new()
                                    .insert("name", "Phil")
                                    .insert("c",
                                            HashBuilder::new()
                                                .insert("d",
                                                        HashBuilder::new()
                                                            .insert("e", HashBuilder::new())))));

        let key = "name".to_string();
        let sections = vec!["a".to_string(),
                            "b".to_string(),
                            "c".to_string(),
                            "d".to_string(),
                            "e".to_string()];
        let data = hb.data;

        let answer = Template::new().look_up_section_data(&key, &sections, &data);

        assert!(answer.is_some());
        match answer {
            Some(d) => {
                match *d {
                    Strng(ref s) => assert_eq!("Phil".to_string(), *s),
                    _ => {
                        assert!(false);
                    }
                }
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_look_up_section_data_also() {
        let hb = HashBuilder::new()
            .insert("a", HashBuilder::new())
            .insert("b", HashBuilder::new().insert("name", "Phil"))
            .insert("c", HashBuilder::new())
            .insert("d", HashBuilder::new())
            .insert("e", HashBuilder::new());

        let key = "name".to_string();
        let sections = vec!["a".to_string(),
                            "b".to_string(),
                            "c".to_string(),
                            "d".to_string(),
                            "e".to_string()];
        let data = hb.data;

        let answer = Template::new().look_up_section_data(&key, &sections, &data);

        assert!(answer.is_some());
        match answer {
            Some(d) => {
                match *d {
                    Strng(ref s) => assert_eq!("Phil".to_string(), *s),
                    _ => {
                        assert!(false);
                    }
                }
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_escape_html() {
        let s1 = "a < b > c & d \"spam\"\'";
        let a1 = "a &lt; b &gt; c &amp; d &quot;spam&quot;'";
        let s2 = "1<2 <b>hello</b>";
        let a2 = "1&lt;2 &lt;b&gt;hello&lt;/b&gt;";

        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Value("value", "{{ value }}".to_string())];
        let data = HashBuilder::new().insert("value", s1);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!(a1, str::from_utf8(w.get_ref()).unwrap());

        w = Cursor::new(Vec::new());
        let newdata = HashBuilder::new().insert("value", s2);
        let rv = Template::new().render_data(&mut w, &newdata, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!(a2, str::from_utf8(w.get_ref()).unwrap());
    }

    #[test]
    fn test_section_tag_iteration() {
        let mut w = Cursor::new(Vec::new());
        let template = "{{#repo}}<b>{{name}}</b>{{/repo}}";
        let tokens = compiler::create_tokens(template);
        let nodes = parser::parse_nodes(&tokens);
        let data = HashBuilder::new().insert("repo",
                                             VecBuilder::new()
                                                 .push(HashBuilder::new()
                                                     .insert("name", "resque"))
                                                 .push(HashBuilder::new().insert("name", "hub"))
                                                 .push(HashBuilder::new().insert("name", "rip")));

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("<b>resque</b><b>hub</b><b>rip</b>".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_not_escape_html() {
        let s = "1<2 <b>hello</b>";
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Unescaped("value", "{{ value }}".to_string())];
        let data = HashBuilder::new().insert("value", s);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!(s, String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_render_to_io_stream() {
        let mut w = Cursor::new(Vec::new());
        let data = HashBuilder::new().insert("value1", "The heading");
        let nodes: Vec<Node> =
            vec![Static("<h1>"), Value("value1", "{{ value1 }}".to_string()), Static("</h1>")];

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("<h1>The heading</h1>".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_bool_false_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> =
            vec![Static("<h1>"), Unescaped("value1", "{{& value1 }}".to_string()), Static("</h1>")];
        let data = HashBuilder::new().insert("value1", false);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("<h1>false</h1>".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_bool_true_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> =
            vec![Static("<h1>"), Unescaped("value1", "{{& value1 }}".to_string()), Static("</h1>")];
        let data = HashBuilder::new().insert("value1", true);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("<h1>true</h1>".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_section_value_string_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Section("value1",
                                            vec![Value("value", "{{ value }}".to_string())],
                                            false,
                                            "{{# value1 }}".to_string(),
                                            "{{/ value1 }}".to_string())];
        let data = HashBuilder::new().insert("value1",
                                             HashBuilder::new().insert("value", "<Section Value>"));

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("&lt;Section Value&gt;".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_section_multiple_value_string_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Section("names",
                                            vec![Value("name", "{{ name }}".to_string())],
                                            false,
                                            "{{# names }}".to_string(),
                                            "{{/ names }}".to_string())];
        let data = HashBuilder::new().insert("names",
                                             HashBuilder::new().insert("name",
                                                                       VecBuilder::new()
                                                                           .push("tom")
                                                                           .push("robert")
                                                                           .push("joe")));

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("tomrobertjoe".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    // #[test]
    // fn test_excessively_nested_data() {
    //     let mut w = Cursor::new(Vec::new());
    //     let nodes: Vec<Node> = vec![Section("hr", vec![Section("people", vec![Value("name", "{{ name }}".to_string())], false, "{{# people }}".to_string(), "{{/ people }}".to_string())], false, "{{# hr }}".to_string(), "{{/ hr }}".to_string())];
    //     let data = HashBuilder::new()
    //         .insert_hash("hr", |builder| {
    //             builder.insert_hash("people", |builder| {
    //                 builder
    //                     .insert_vector("name", |builder| {
    //                         builder
    //                             .push("tom")
    //                             .push("robert")
    //                             .push("joe")
    //                 })
    //             })
    //         });

    //     let rv = Template::new().render_data(&mut w, &data, &nodes);
    //     assert_eq!("tomrobertjoe".to_string(), String::from_utf8(w.into_inner()).unwrap());
    // }

    #[test]
    fn test_unescaped_node_lambda_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> =
            vec![Static("<h1>"), Unescaped("func1", "{{& func1 }}".to_string()), Static("</h1>")];
        let mut f = |_| "heading".to_string();
        let data = HashBuilder::new().insert_lambda("func1", &mut f);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("<h1>heading</h1>".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_value_node_lambda_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> =
            vec![Static("<h1>"), Value("func1", "{{ func1 }}".to_string()), Static("</h1>")];
        let mut f = |_| "heading".to_string();
        let data = HashBuilder::new().insert_lambda("func1", &mut f);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("<h1>heading</h1>".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    // #[test]
    // fn test_spec_lambdas_interpolation_using_render_text() {
    //     let mut s = Cursor::new(Vec::new());
    //     let data = HashBuilder::new()
    //                 .insert_lambda("lambda", |_| {
    //                      "world".to_string()
    //                  });
    //     let s = rustache::render_text("Hello, {{lambda}}!", data);
    //     assert_eq!("Hello, world!".to_string(), String::from_utf8(w.into_inner()).unwrap());
    // }

    // #[test]
    // fn test_spec_lambdas_inverted_section_using_render_text() {
    //     let data = HashBuilder::new()
    //                 .insert("static", "static")
    //                 .insert_lambda("lambda", |_| {
    //                     "false".to_string()
    //                 });

    //     let s = rustache::render_text("<{{^lambda}}{{static}}{{/lambda}}>", data);

    //     assert_eq!("<>".to_string(), String::from_utf8(w.into_inner()).unwrap());
    // }

    #[test]
    fn test_value_node_correct_false_bool_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Value("value1", "{{ value1 }}".to_string())];
        let data = HashBuilder::new().insert("value1", false);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("false".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_value_node_correct_true_bool_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Value("value1", "{{ value1 }}".to_string())];
        let data = HashBuilder::new().insert("value1", true);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!("true".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_partial_node_correct_data() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Static("A wise woman once said: "),
                                    Part("hopper_quote.partial", "{{> hopper_quote.partial }}")];
        let data = HashBuilder::new()
            .insert("author", "Grace Hopper")
            .set_partials_path("test_data");

        let mut s: String = String::new();
        s.push_str("A wise woman once said: It's easier to get forgiveness than \
                    permission.-Grace Hopper");

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!(s, String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_partial_node_correct_data_with_extra() {
        let mut w = Cursor::new(Vec::new());
        let nodes: Vec<Node> = vec![Static("A wise woman once said: "),
                                    Part("hopper_quote.partial", "{{> hopper_quote.partial }}"),
                                    Static(" something else "),
                                    Value("extra", "{{ extra }}".to_string())];
        let data = HashBuilder::new()
            .insert("author", "Grace Hopper")
            .insert("extra", "extra data")
            .set_partials_path("test_data");

        let mut s: String = String::new();
        s.push_str("A wise woman once said: It's easier to get forgiveness than \
                    permission.-Grace Hopper something else extra data");

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        assert_eq!(s, String::from_utf8(w.into_inner()).unwrap());
    }

    #[test]
    fn test_section_node_partial_node_correct_data() {
        let mut w = Cursor::new(Vec::new());
        let data = HashBuilder::new()
            .set_partials_path("test_data")
            .insert("people",
                    HashBuilder::new().insert("information",
                                              VecBuilder::new()
                                                  .push("<tr><td>Jarrod</td><td>Ruhland</td></tr>")
                                                  .push("<tr><td>Sean</td><td>Chen</td></tr>")
                                                  .push("<tr><td>Fleur</td><td>Dragan</td></tr>")
                                                  .push("<tr><td>Jim</td><td>O'Brien</td></tr>")));
        let path = Path::new("test_data/section_with_partial_template.html");
        let contents = File::open(path)
            .and_then(|mut fp| {
                let mut contents = String::new();
                fp.read_to_string(&mut contents)
                    .map(move |_| contents)
            }).unwrap();

        let mut tokens = compiler::create_tokens(&contents[..]);
        let nodes = parser::parse_nodes(&mut tokens);

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }

        let mut f = File::create(&Path::new("test_data/section_with_partial.html")).unwrap();
        w.seek(SeekFrom::Start(0)).unwrap();
        let completed = io::copy(&mut w, &mut f);
        assert_eq!(completed.is_ok(), true);
    }

    // - name: Interpolation - Multiple Calls
    //   desc: Interpolated lambdas should not be cached.
    //   data:
    //     lambda: !code
    //       ruby:    'proc { $calls ||= 0; $calls += 1 }'
    //       perl:    'sub { no strict; $calls += 1 }'
    //       js:      'function() {return (g=(function(){return this})()).calls=(g.calls||0)+1 }'
    //       php:     'global $calls; return ++$calls;'
    //       python:  'lambda: globals().update(calls=globals().get("calls",0)+1) or calls'
    //       clojure: '(def g (atom 0)) (fn [] (swap! g inc))'
    //   template: '{{lambda}} == {{{lambda}}} == {{lambda}}'
    //   expected: '1 == 2 == 3'
    #[test]
    fn test_spec_lambda_not_cached_on_interpolation() {
        let mut planets = vec!["Jupiter", "Earth", "Saturn"];
        let mut w = Cursor::new(Vec::new());
        let mut tokens = compiler::create_tokens("{{lambda}} == {{&lambda}} == {{lambda}}");
        let nodes = parser::parse_nodes(&mut tokens);
        let mut f = |_| planets.pop().unwrap().to_string();
        let data = HashBuilder::new()
            .insert_lambda("lambda", &mut f)
            .insert("planet", "world");

        let rv = Template::new().render_data(&mut w, &data, &nodes);
        match rv {
            _ => {}
        }
        assert_eq!("Saturn == Earth == Jupiter".to_string(),
                   String::from_utf8(w.into_inner()).unwrap());
    }

}
