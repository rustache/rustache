use std::path::Path;
use parser::{Parser, Node, Value, Static, Unescaped, Section, Part};
use super::{Data, Strng, Bool, Vector, Hash, Func, Read};
use build::HashBuilder;
use compiler::Compiler;
use std::collections::HashMap;

pub struct Template<'a> {
   partials_path: String
}

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        let tmpl = Template {
            partials_path: String::new()
        };
        tmpl
    }  

    fn escape_html(&self, input: &str) -> Box<String> {
        let mut rv = box String::new();
        for c in input.chars() {
            match c {
                '<'  => { rv.push_str("&lt;"); }
                '>'  => { rv.push_str("&gt;"); }
                '&'  => { rv.push_str("&amp;"); }
                '"'  => { rv.push_str("&quot;"); }
                _    => { rv.push(c); }
            }
        }
        rv
    }

    fn handle_unescaped_node<'a, W: Writer>(&self, data: &Data, key: String, writer: &mut W) {
        let mut tmp: String = String::new();
        match *data {
            Strng(ref val) => {
                tmp = tmp + *val;
                writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
            },
            Bool(ref val) => {
                match val {
                    &true  => tmp.push_str("true"),
                    &false => tmp.push_str("false")
                }
                writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
            },
            Vector(ref list) => {
                for item in list.iter() {
                    self.handle_unescaped_node(item, key.to_string(), writer);
                }
            },
            Hash(ref hash) => {
                if hash.contains_key(&key) {
                    let ref tmp = hash[key];
                    self.handle_unescaped_node(tmp, key.to_string(), writer);
                }
            },
            /// Should return the String representation of the function without evaluation
           Func(ref f) => {
                let f = &mut *f.borrow_mut();
                let val = (*f)("".to_string());
                writer.write_str(val.as_slice()).ok().expect("write failed in render");
            }

        }
    }

    fn handle_value_node<'a, W: Writer>(&self, data: &Data, key: String, writer: &mut W) {
        let mut tmp: String = String::new();
        match *data {
            Strng(ref val) => {
                tmp = *self.escape_html(&(*val.as_slice()));
                writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
            },
            Bool(ref val) => {
                match val {
                    &true  => tmp.push_str("true"),
                    &false => tmp.push_str("false")
                }
                writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
            },
            Vector(ref list) => {
                for item in list.iter() {
                    self.handle_value_node(item, key.to_string(), writer);
                }
            },
            Hash(ref hash) => {
                if hash.contains_key(&key) {
                    let ref tmp = hash[key];
                    self.handle_value_node(tmp, key.to_string(), writer);
                }
            },
            /// Should evaluate the function and return its result
            Func(ref f) => {
                let f = &mut *f.borrow_mut();
                let val = (*f)("".to_string());
                let value = self.escape_html(val.as_slice());
                writer.write_str(value.as_slice()).ok().expect("write failed in render");
            }
        }       
    }

    fn handle_inverted_node<'a, W:Writer>(&mut self, nodes: &Vec<Node>, data: &HashMap<String, Data>, writer: &mut W) {
        for node in nodes.iter() {
            match *node {
                Static(key) => {
                    writer.write_str(key.as_slice()).ok().expect("write failed in render");
                },
                Part(filename) => {
                    self.handle_partial_file_node(filename, data, writer);
                },
                _ => {}
            }
        }
    }

    fn handle_section_node<'a, W: Writer>(&mut self, nodes: &Vec<Node>, data: &Data, datastore: &HashMap<String,Data>, writer: &mut W) {
        for node in nodes.iter() {
            match *node {
                Unescaped(key)  => {
                    self.handle_unescaped_node(data, key.to_string(), writer);
                }
                Value(key) => {
                    self.handle_value_node(data, key.to_string(), writer);
                }
                Static(key) => {
                    writer.write_str(key.as_slice()).ok().expect("write failed in render");
                }
                Section(ref key, ref children, ref inverted) => {
                    match inverted {
                        &false => {
                            match *data {
                                Hash(ref hash) => {
                                    self.handle_section_node(children, &hash[key.to_string()], datastore, writer);        
                                },
                                _ => {
                                    self.handle_section_node(children, data, datastore, writer);
                                }
                            }
                        },
                        &true => {
                            self.handle_inverted_node(children, datastore, writer);
                        }
                    }
                },
                Part(path) => {
                    self.handle_partial_file_node(path, datastore, writer);
                }
            }
        }
    }

   fn handle_partial_file_node<'a, W: Writer>(&mut self,
                                              filename: &str, 
                                                  data: &HashMap<String, Data>, 
                                                writer: &mut W) {
        let tmp = Path::new(self.partials_path.clone()).join(filename);
        match tmp.as_str() {
            None => fail!("path is not a valid UTF-8 sequence"),
            Some(_) => {
                let file = Read::read_file(tmp.clone());
                let compiler = Compiler::new(file.as_slice());
                let parser = Parser::new(&compiler.tokens);

                self.render(writer, data, &parser);
            }
        }
    }

    // writer: an io::stream to write the rendered template out to
    // data:   the internal HashBuilder data store
    // parser: the parser object that has the parsed nodes, see src/parse.js
    pub fn render<'a, W: Writer>(&mut self, writer: &mut W, data: &HashMap<String, Data>, parser: &Parser) {
        let mut tmp: String = String::new();

        // nodes are what the template file is parsed into
        // we have to iterate through each one and handle it as
        // the kind of node it is
        for node in parser.nodes.iter() {
            tmp.truncate(0);
            match *node {
                // unescaped nodes contain tags who's data gets written
                // out exactly as provided, no HTML escaping
                Unescaped(key)  => {
                    let tmp = key.to_string();
                    if data.contains_key(&tmp) {
                        let ref val = data[tmp];
                        self.handle_unescaped_node(val, "".to_string(), writer);
                    }
                }
                // value nodes contain tags who's data gets HTML escaped
                // when it gets written out
                Value(key) => {
                    let tmp = key.to_string();
                    if data.contains_key(&tmp) {
                        let ref val = data[tmp];
                        self.handle_value_node(val, "".to_string(), writer);
                    }
                }
                // static nodes are the test in the template that doesn't get modified, 
                // just gets written out character for character
                Static(key) => {
                    writer.write_str(key).ok().expect("write failed in render");
                }
                // sections come in two kinds, normal and inverted
                //
                // inverted are if the tag data is not there, the Static between it 
                // and it's closing tag gets written out, otherwise the text is thrown out
                //
                // normal section tags enclose a bit of html that will get repeated
                // for each element found in it's data
                Section(ref key, ref children, ref inverted) => {
                    let tmp = key.to_string();
                    match (data.contains_key(&tmp), *inverted) {
                        (true, true) => {},
                        (false, false) => {},
                        (true, false) => {
                            let ref val = data[tmp];
                            self.handle_section_node(children, val, data, writer);
                        },
                        (false, true) => {
                            let ref val = data[tmp];
                            self.handle_inverted_node(children, data, writer);
                        }
                    }
                }
                // partials include external template files and compile and process them
                // at runtime, inserting them into the document at the point the tag is found
                Part(name) => {
                    self.handle_partial_file_node(name, data, writer);
                }
            }
        }
    }

    // main entry point to Template
    pub fn render_data<'a, W: Writer>(&mut self, writer: &mut W, datastore: &HashBuilder<'a>, parser: &Parser) {
        // we need to hang on to the partials path internally,
        // if there is one, for class methods to use.
        self.partials_path.truncate(0);
        self.partials_path.push_str(datastore.partials_path);

        self.render(writer, &datastore.data, parser);
    }

}

#[cfg(test)]
mod template_tests {
    use std::io::File;
    use std::io::MemWriter;
    use std::str;

    use parser::Parser;
    use template::Template;
    use compiler::Compiler;
    use build::HashBuilder;
    use super::super::Read;

    #[test]
    fn test_escape_html() {
        let s1 = "a < b > c & d \"spam\"\'";
        let a1 = "a &lt; b &gt; c &amp; d &quot;spam&quot;'";
        let s2 = "1<2 <b>hello</b>";
        let a2 = "1&lt;2 &lt;b&gt;hello&lt;/b&gt;";

        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{ value }}");
        let parser = Parser::new(&compiler.tokens);
        let mut data = HashBuilder::new().insert_string("value", s1);
        Template::new().render_data(&mut w, &data, &parser);
        assert_eq!(a1, str::from_utf8(w.get_ref()).unwrap());

        w = MemWriter::new();
        data = HashBuilder::new().insert_string("value", s2);
        Template::new().render_data(&mut w, &data, &parser);
        assert_eq!(a2, str::from_utf8(w.get_ref()).unwrap());
    }

    #[test]
    fn test_not_escape_html() {
        let s2 = "1<2 <b>hello</b>";
        let mut w = MemWriter::new();
        let compiler2 = Compiler::new("{{& value }}");

        let parser = Parser::new(&compiler2.tokens);
        let data = HashBuilder::new().insert_string("value", s2);

        Template::new().render_data(&mut w, &data, &parser);
        assert_eq!(s2, str::from_utf8(w.get_ref()).unwrap());        
    }

    #[test]
    fn test_render_to_io_stream() {
        let mut w = MemWriter::new();

        let data = HashBuilder::new().insert_string("value1", "The heading");

        let compiler = Compiler::new("<h1>{{ value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);

        Template::new().render_data(&mut w, &data, &parser);
        assert_eq!("<h1>The heading</h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_string_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("value1", "heading");

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<h1>heading</h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_html_string_data() {
        let s1 = "a < b > c & d \"spam\"\'";
        let a1 = "<h1>a < b > c & d \"spam\"\'</h1>";
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("value1", s1);

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!(a1.to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_bool_false_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", false);

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<h1>false</h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_bool_true_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", true);

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<h1>true</h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }


    #[test]
    fn test_section_unescaped_string_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{# value1 }}{{& value }}{{/ value1}}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new()
            .insert_hash("value1", |builder| {
                builder.insert_string("value", "<Section Value>")
            });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<Section Value>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_section_value_string_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{# value1 }}{{ value }}{{/ value1 }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new()
            .insert_hash("value1", |builder| {
                builder.insert_string("value", "<Section Value>")
            });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("&lt;Section Value&gt;".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_section_multiple_value_string_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{# names }}{{ name }}{{/ names }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new()
            .insert_hash("names", |builder| {
                builder.insert_vector("name", |builder| {
                    builder
                        .push_string("tom")
                        .push_string("robert")
                        .push_string("joe")
                })
            });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("tomrobertjoe".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_excessively_nested_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{# hr }}{{# people }}{{ name }}{{/ people }}{{/ hr }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new()
            .insert_hash("hr", |builder| {
                builder.insert_hash("people", |builder| {
                    builder
                        .insert_vector("name", |builder| {
                            builder
                                .push_string("tom")
                                .push_string("robert")
                                .push_string("joe")
                    })
                })
            });

        Template::new().render_data(&mut w, &data, &parser);
        assert_eq!("tomrobertjoe".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }    

    #[test]
    fn test_value_node_correct_html_string_data() {
        let s1 = "a < b > c & d \"spam\"\'";
        let a1 = "a &lt; b &gt; c &amp; d &quot;spam&quot;'";
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{ value1 }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("value1", s1);

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!(a1.to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_correct_string_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{ value1 }}<h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("value1", "heading");

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<h1>heading<h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_lambda_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& func1 }}<h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_func("func1", |_| {
            "heading".to_string()
        });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<h1>heading<h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_lambda_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{ func1 }}<h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_func("func1", |_| {
            "heading".to_string()
        });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<h1>heading<h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_correct_html_string_lambda_data() {
        let s1 = "a < b > c & d \"spam\"\'";
        let a1 = "a &lt; b &gt; c &amp; d &quot;spam&quot;'";
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{ func1 }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_func("func1", |_| {
            s1.to_string()
        });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!(a1.to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_correct_false_bool_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{ value1 }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", false);

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("false".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_correct_true_bool_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{ value1 }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", true);

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("true".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_partial_node_correct_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("A wise woman once said: {{> hopper_quote.partial }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("author", "Grace Hopper")
                                     .set_partials_path("test_data");

        let mut s: String = String::new();
        s.push_str("A wise woman once said: It's easier to get forgiveness than permission.-Grace Hopper");

        Template::new().render_data(&mut w, &data, &parser);
        assert_eq!(s, String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_partial_node_correct_data_with_extra() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("A wise woman once said: {{> hopper_quote.partial }} something else {{ extra }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("author", "Grace Hopper")
                                     .insert_string("extra", "extra data")
                                     .set_partials_path("test_data");

        let mut s: String = String::new();
        s.push_str("A wise woman once said: It's easier to get forgiveness than permission.-Grace Hopper something else extra data");

        Template::new().render_data(&mut w, &data, &parser);
        assert_eq!(s, String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_section_node_partial_node_correct_data() {
        let mut w = MemWriter::new();
        let data = HashBuilder::new()
            .set_partials_path("test_data")
            .insert_hash("people", |builder| {
                builder.insert_vector("information", |builder| {
                    builder
                        .push_string("<tr><td>Jarrod</td><td>Ruhland</td></tr>")
                        .push_string("<tr><td>Sean</td><td>Chen</td></tr>")
                        .push_string("<tr><td>Fleur</td><td>Dragan</td></tr>")
                        .push_string("<tr><td>Jim</td><td>O'Brien</td></tr>")
                    }
                )}
            );

        let file = Read::read_file(Path::new("test_data/section_with_partial_template.html"));
        let compiler = Compiler::new(file.as_slice());
        let parser = Parser::new(&compiler.tokens);

        Template::new().render_data(&mut w, &data, &parser);

        let mut f = File::create(&Path::new("test_data/section_with_partial.html"));
        let completed = f.write(w.unwrap().as_slice());
        assert_eq!(completed, Ok(()));


        //assert_eq!(s, String::from_utf8(w.unwrap()).unwrap());
    }
}
