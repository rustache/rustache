use std::path::Path;
use parser::{Parser, Node, Value, Static, Unescaped, Section, Part};
use super::{Data, Strng, Bool, Vector, Hash, Lambda};
use build::HashBuilder;
use compiler::Compiler;
use std::collections::HashMap;
use rustache::Rustache;
use std::io::stdio::stdout;

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

    fn write_to_stream<'a, W: Writer>(&self, writer: &mut W, data: &str, errstr: &str) {

        let rv = writer.write_str(data);
        match rv {
            Err(err) => {
                let msg = format!("{}: {}", err, errstr);
                fail!(msg);
            }
            Ok(_) => { }
        }
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

    fn handle_unescaped_lambda_interpolation<W: Writer>(&mut self, f: &mut |String|: 'a -> String, data: &HashMap<String, Data>, raw: String, writer: &mut W) {
        let val = (*f)(raw);
        let compiler = Compiler::new(val.as_slice());
        let parser = Parser::new(&compiler.tokens);

        self.render(writer, data, &parser);
    }

    fn handle_escaped_lambda_interpolation<W: Writer>(&mut self, f: &mut |String|: 'a -> String, data: &HashMap<String, Data>, raw: String, writer: &mut W) {
        let val = (*f)(raw);
        let value = self.escape_html(val.as_slice());
        let compiler = Compiler::new(value.as_slice());
        let parser = Parser::new(&compiler.tokens);

        self.render(writer, data, &parser);
    }

    fn handle_unescaped_node<'a, W: Writer>(&mut self, data: &Data, key: String, datastore: &HashMap<String, Data>, writer: &mut W) {
        let mut tmp: String = String::new();
        match *data {
            Strng(ref val) => {
                tmp = tmp + *val;
                self.write_to_stream(writer, tmp.as_slice(), "render: unescaped node string fail");
            },
            Bool(ref val) => {
                match val {
                    &true  => tmp.push_str("true"),
                    &false => tmp.push_str("false")
                }
                self.write_to_stream(writer, tmp.as_slice(), "render: unescaped node bool");
            },
            Vector(ref list) => {
                for item in list.iter() {
                    self.handle_unescaped_node(item, key.to_string(), datastore, writer);
                }
            },
            Hash(ref hash) => {
                if hash.contains_key(&key) {
                    let ref tmp = hash[key];
                    self.handle_unescaped_node(tmp, key.to_string(), datastore, writer);
                }
            },
            Lambda(ref f) => {
                let raw = "".to_string();
                self.handle_unescaped_lambda_interpolation(&mut *f.borrow_mut(), datastore, raw, writer);
            }

        }
    }

    fn handle_value_node<'a, W: Writer>(&mut self, data: &Data, key: String, datastore: &HashMap<String, Data>, writer: &mut W) {
        let mut tmp: String = String::new();
        match *data {
            Strng(ref val) => {
                tmp = *self.escape_html(&(*val.as_slice()));
                self.write_to_stream(writer, tmp.as_slice(), "render: value node string");
            },
            Bool(ref val) => {
                match val {
                    &true  => tmp.push_str("true"),
                    &false => tmp.push_str("false")
                }
                self.write_to_stream(writer, tmp.as_slice(), "render: value node bool");
            },
            Vector(ref list) => {
                for item in list.iter() {
                    self.handle_value_node(item, key.to_string(), datastore, writer);
                }
            },
            Hash(ref hash) => {
                if hash.contains_key(&key) {
                    let ref tmp = hash[key];
                    self.handle_value_node(tmp, key.to_string(), datastore, writer);
                }
            },
            Lambda(ref f) => {
                let raw = "".to_string();
                self.handle_escaped_lambda_interpolation(&mut *f.borrow_mut(), datastore, raw, writer);
            }
        }       
    }

    fn handle_inverted_node<'a, W:Writer>(&mut self, nodes: &Vec<Node>, data: &HashMap<String, Data>, writer: &mut W) {
        for node in nodes.iter() {
            match *node {
                Static(key) => {
                    self.write_to_stream(writer, key.as_slice(), "render: inverted node static");
                },
                Part(filename, _) => {
                    self.handle_partial_file_node(filename, data, writer);
                },
                _ => {}
            }
        }
    }

    fn handle_section_node<'a, W: Writer>(&mut self, nodes: &Vec<Node>, data: &Data, datastore: &HashMap<String,Data>, writer: &mut W) {
        for node in nodes.iter() {
            match *node {
                Unescaped(key, _)  => {
                    self.handle_unescaped_node(data, key.to_string(), datastore, writer);
                }
                Value(key, _) => {
                    self.handle_value_node(data, key.to_string(), datastore, writer);
                }
                Static(key) => {
                    self.write_to_stream(writer, key.as_slice(), "render: section node static");
                }
                Section(ref key, ref children, ref inverted, open, close) => {
                    match inverted {
                        &false => {
                            match *data {
                                Hash(ref hash) => {
                                    self.handle_section_node(children, &hash[key.to_string()], datastore, writer);        
                                },
                                Lambda(ref f) => {
                                    let raw = self.get_section_text(children);
                                    self.handle_unescaped_lambda_interpolation(&mut *f.borrow_mut(), datastore, *raw, writer);
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
                Part(path, _) => {
                    self.handle_partial_file_node(path, datastore, writer);
                }
            }
        }
    }

    fn get_section_text(&self, children: &Vec<Node>) -> Box<String> {
        let mut temp = box String::new();
        for child in children.iter() {
            match child {
                &Static(text) => temp.push_str(text),
                &Value(_, text) => temp.push_str(text),
                &Section(_, ref children, _, open, close) => {
                    let rv = self.get_section_text(children);
                    temp.push_str(open);
                    temp.push_str(rv.as_slice());
                    temp.push_str(close);
                },
                &Unescaped(_, text) => temp.push_str(text),
                &Part(_, text) => temp.push_str(text)
            }
        }
        stdout().write_str("TEMP");
        stdout().write_str(temp.as_slice());
        temp
    }

    fn handle_partial_file_node<'a, W: Writer>(&mut self,
                                              filename: &str, 
                                                  data: &HashMap<String, Data>, 
                                                writer: &mut W) {
        let tmp = Path::new(self.partials_path.clone()).join(filename);
        match tmp.as_str() {
            None => fail!("path is not a valid UTF-8 sequence"),
            Some(_) => {
                let file = Rustache::read_file(tmp.clone());
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
                Unescaped(key, _)  => {
                    let tmp = key.to_string();
                    if data.contains_key(&tmp) {
                        let ref val = data[tmp];
                        self.handle_unescaped_node(val, "".to_string(), data, writer);
                    }
                }
                // value nodes contain tags who's data gets HTML escaped
                // when it gets written out
                Value(key, _) => {
                    let tmp = key.to_string();
                    if data.contains_key(&tmp) {
                        let ref val = data[tmp];
                        self.handle_value_node(val, "".to_string(), data, writer);
                    }
                }
                // static nodes are the test in the template that doesn't get modified, 
                // just gets written out character for character
                Static(key) => {
                    self.write_to_stream(writer, key, "render: static");
                    //writer.write_str(key).ok().expect("write failed in render");
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
                Part(name, _) => {
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
    use std::io::stdio::stdout;
    use std::io::File;
    use std::io::MemWriter;
    use std::str;

    use parser::Parser;
    use rustache::Rustache;
    use template::Template;
    use compiler::Compiler;
    use build::HashBuilder;

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
        let data = HashBuilder::new().insert_lambda("func1", |_| {
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
        let data = HashBuilder::new().insert_lambda("func1", |_| {
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
        let data = HashBuilder::new().insert_lambda("func1", |_| {
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

        let file = Rustache::read_file(Path::new("test_data/section_with_partial_template.html"));
        let compiler = Compiler::new(file.as_slice());
        let parser = Parser::new(&compiler.tokens);

        Template::new().render_data(&mut w, &data, &parser);

        let mut f = File::create(&Path::new("test_data/section_with_partial.html"));
        let completed = f.write(w.unwrap().as_slice());
        assert_eq!(completed, Ok(()));
    }

  // - name: Interpolation
  //   desc: A lambda's return value should be interpolated.
  //   data:
  //     lambda: !code
  //       ruby:    'proc { "world" }'
  //       perl:    'sub { "world" }'
  //       js:      'function() { return "world" }'
  //       php:     'return "world";'
  //       python:  'lambda: "world"'
  //       clojure: '(fn [] "world")'
  //   template: "Hello, {{lambda}}!"
  //   expected: "Hello, world!"

    #[test]
    fn test_spec_lambda_return_value_interpolated() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("Hello, {{lambda}}!");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_lambda("lambda", |_| { "world".to_string() });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("Hello, world!".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

  // - name: Interpolation - Expansion
  //   desc: A lambda's return value should be parsed.
  //   data:
  //     planet: "world"
  //     lambda: !code
  //       ruby:    'proc { "{{planet}}" }'
  //       perl:    'sub { "{{planet}}" }'
  //       js:      'function() { return "{{planet}}" }'
  //       php:     'return "{{planet}}";'
  //       python:  'lambda: "{{planet}}"'
  //       clojure: '(fn [] "{{planet}}")'
  //   template: "Hello, {{lambda}}!"
  //   expected: "Hello, world!"

    #[test]
    fn test_spec_lambda_return_value_parsed() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("Hello, {{lambda}}!");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_lambda("lambda", |_| { "{{planet}}".to_string() })
                                     .insert_string("planet", "world");

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("Hello, world!".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

  // - name: Interpolation - Multiple Calls
  //   desc: Interpolated lambdas should not be cached.
  //   data:
  //     lambda: !code
  //       ruby:    'proc { $calls ||= 0; $calls += 1 }'
  //       perl:    'sub { no strict; $calls += 1 }'
  //       js:      'function() { return (g=(function(){return this})()).calls=(g.calls||0)+1 }'
  //       php:     'global $calls; return ++$calls;'
  //       python:  'lambda: globals().update(calls=globals().get("calls",0)+1) or calls'
  //       clojure: '(def g (atom 0)) (fn [] (swap! g inc))'
  //   template: '{{lambda}} == {{{lambda}}} == {{lambda}}'
  //   expected: '1 == 2 == 3'

    #[test]
    fn test_spec_lambda_not_cached_on_interpolation() {
        let mut planets = vec!["Jupiter", "Earth", "Saturn"];
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{lambda}} == {{&lambda}} == {{lambda}}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_lambda("lambda", |_| { planets.pop().unwrap().to_string() } )
                                     .insert_string("planet", "world");

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("Saturn == Earth == Jupiter".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

  // - name: Escaping
  //   desc: Lambda results should be appropriately escaped.
  //   data:
  //     lambda: !code
  //       ruby:    'proc { ">" }'
  //       perl:    'sub { ">" }'
  //       js:      'function() { return ">" }'
  //       php:     'return ">";'
  //       python:  'lambda: ">"'
  //       clojure: '(fn [] ">")'
  //   template: "<{{lambda}}{{{lambda}}}"
  //   expected: "<&gt;>"

    #[test]
    fn test_spec_lambda_results_appropriately_escaped() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<{{lambda}}{{&lambda}}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_lambda("lambda", |_| { return ">".to_string() });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<&gt;>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    // - name: Section
    //   desc: Lambdas used for sections should receive the raw section string.
    //   data:
    //     x: 'Error!'
    //     lambda: !code
    //       ruby:    'proc { |text| text == "{{x}}" ? "yes" : "no" }'
    //       perl:    'sub { $_[0] eq "{{x}}" ? "yes" : "no" }'
    //       js:      'function(txt) { return (txt == "{{x}}" ? "yes" : "no") }'
    //       php:     'return ($text == "{{x}}") ? "yes" : "no";'
    //       python:  'lambda text: text == "{{x}}" and "yes" or "no"'
    //       clojure: '(fn [text] (if (= text "{{x}}") "yes" "no"))'
    //   template: "<{{#lambda}}{{x}}{{/lambda}}>"
    //   expected: "<yes>"

    #[test]
    fn test_spec_lambdas_receive_raw_section_string() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<{{#lambda}}{{x}}{{/lambda}}>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_lambda("lambda", |text| { stdout().write_str("OUTPUT"); 
            stdout().write_str(text.as_slice()); 
            if text.as_slice() == "{{x}}" { "yes".to_string() } else { "no".to_string() } });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<yes>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    // - name: Section - Expansion
    //   desc: Lambdas used for sections should have their results parsed.
    //   data:
    //     planet: "Earth"
    //     lambda: !code
    //       ruby:    'proc { |text| "#{text}{{planet}}#{text}" }'
    //       perl:    'sub { $_[0] . "{{planet}}" . $_[0] }'
    //       js:      'function(txt) { return txt + "{{planet}}" + txt }'
    //       php:     'return $text . "{{planet}}" . $text;'
    //       python:  'lambda text: "%s{{planet}}%s" % (text, text)'
    //       clojure: '(fn [text] (str text "{{planet}}" text))'
    //   template: "<{{#lambda}}-{{/lambda}}>"
    //   expected: "<-Earth->"

    #[test]
    fn test_spec_lambdas_for_sections_parsed() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<{{#lambda}}-{{/lambda}}>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_lambda("lambda", |_| { "#{text}{{planet}}#{text}".to_string() })
                                     .insert_string("planet", "Earth");

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("<-Earth->".to_string(), String::from_utf8(w.unwrap()).unwrap());

    }

    // - name: Section - Multiple Calls
    //   desc: Lambdas used for sections should not be cached.
    //   data:
    //     lambda: !code
    //       ruby:    'proc { |text| "__#{text}__" }'
    //       perl:    'sub { "__" . $_[0] . "__" }'
    //       js:      'function(txt) { return "__" + txt + "__" }'
    //       php:     'return "__" . $text . "__";'
    //       python:  'lambda text: "__%s__" % (text)'
    //       clojure: '(fn [text] (str "__" text "__"))'
    //   template: '{{#lambda}}FILE{{/lambda}} != {{#lambda}}LINE{{/lambda}}'
    //   expected: '__FILE__ != __LINE__'

    #[test]
    fn test_spec_lambdas_for_sections_not_cached() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{#lambda}}FILE{{/lambda}} != {{#lambda}}LINE{{/lambda}}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_lambda("lambda", |text| { String::new().append("__").append(text.as_slice()).append("__") });

        Template::new().render_data(&mut w, &data, &parser);

        assert_eq!("__FILE__ != __LINE__".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }
}
