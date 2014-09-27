
use std::path::Path;
use parser::{Parser, Node, Value, Static, Unescaped, Section, File};
use super::{Data, Strng, Bool, Vector, Hash, Read};
use build::HashBuilder;
use compiler::Compiler;

pub struct Template<'a>;

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template
    }

    fn escape_html(input: &str) -> Box<String> {
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

    fn handle_unescaped_node<'a, W: Writer>(data: &Data, key: String, writer: &mut W) {
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
                    Template::handle_unescaped_node(item, key.to_string(), writer);
                }
            },
            Hash(ref hash) => {
                if hash.contains_key(&key) {
                    let ref tmp = hash[key];
                    Template::handle_unescaped_node(tmp, key.to_string(), writer);
                }
            }
        }
    }

    fn handle_value_node<'a, W: Writer>(data: &Data, key: String, writer: &mut W) {
        let mut tmp: String = String::new();
        match *data {
            Strng(ref val) => {
                tmp = *Template::escape_html(&(*val.as_slice()));
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
                    Template::handle_value_node(item, key.to_string(), writer);
                }
            },
            Hash(ref hash) => {
                if hash.contains_key(&key) {
                    let ref tmp = hash[key];
                    Template::handle_value_node(tmp, key.to_string(), writer);
                }
            }
        }       
    }

    fn handle_inverted_node<'a, W:Writer>(nodes: &Vec<Node>, writer: &mut W) {
        for node in nodes.iter() {
            match *node {
                Static(key) => {
                    writer.write_str(key.as_slice()).ok().expect("write failed in render");
                },
                File(path) => {
                    // handle partial logic here...
                },
                _ => {}
            }
        }
    }

    fn handle_section_node<'a, W: Writer>(nodes: &Vec<Node>, data: &Data, writer: &mut W) {
        for node in nodes.iter() {
            match *node {
                Unescaped(key)  => {
                    Template::handle_unescaped_node(data, key.to_string(), writer);
                }
                Value(key) => {
                    Template::handle_value_node(data, key.to_string(), writer);
                }
                Static(key) => {
                    writer.write_str(key.as_slice()).ok().expect("write failed in render");
                }
                Section(ref key, ref children, ref inverted) => {
                    match inverted {
                        &false => {
                            match *data {
                                Hash(ref hash) => {
                                    Template::handle_section_node(children, &hash[key.to_string()], writer);        
                                }
                                _ => {
                                    Template::handle_section_node(children, data, writer);
                                }
                            }
                        },
                        &true => {
                            Template::handle_inverted_node(children, writer);
                        }
                    }
                },
                File(path) => {
                    // handle partial logic here...
                }
            }
        }
    }



    //     // `join` merges a path with a byte container using the OS specific
    // // separator, and returns the new path
    // let new_path = path.join("a").join("b");

    // // Convert the path into a string slice
    // match new_path.as_str() {
    //     None => fail!("new path is not a valid UTF-8 sequence"),
    //     Some(s) => println!("new path is {}", s),
    // }



   fn handle_partial_file_node<'a, W: Writer>(pathname: &str,
                                              filename: &str, 
                                                  data: &HashBuilder, 
                                                writer: &mut W) {
        let tmp = Path::new(pathname).join(filename);
        match tmp.as_str() {
            None => fail!("path is not a valid UTF-8 sequence"),
            Some(path) => {
                let file = Read::read_file(tmp.clone());
                let compiler = Compiler::new(file.as_slice());
                let parser = Parser::new(&compiler.tokens);

                Template::render_data(writer, data, &parser);
            }
        }
    }

 
    pub fn render_data<'a, W: Writer>(writer: &mut W, datastore: &HashBuilder, parser: &Parser) {
        let mut tmp: String = String::new();
        for node in parser.nodes.iter() {
            tmp.truncate(0);
            match *node {
                Unescaped(key)  => {
                    let tmp = key.to_string();
                    if datastore.data.contains_key(&tmp) {
                        let ref val = datastore.data[tmp];
                        Template::handle_unescaped_node(val, "".to_string(), writer);
                    }
                }
                Value(key) => {
                    let tmp = key.to_string();
                    if datastore.data.contains_key(&tmp) {
                        let ref val = datastore.data[tmp];
                        Template::handle_value_node(val, "".to_string(), writer);
                    }
                }
                Static(key) => {
                    tmp.push_str(key);
                    writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
                }
                Section(ref key, ref children, ref inverted) => {
                    let tmp = key.to_string();
                    match (datastore.data.contains_key(&tmp), *inverted) {
                        (true, true) => {},
                        (false, false) => {},
                        (true, false) => {
                            let ref val = datastore.data[tmp];
                            Template::handle_section_node(children, val, writer);
                        },
                        (false, true) => {
                            let ref val = datastore.data[tmp];
                            Template::handle_inverted_node(children, writer);
                        }
                    }
                }
                File(name) => {
                    let path = datastore.partials_path;
                    Template::handle_partial_file_node(path, name, datastore, writer);
                }
            }
        }
    }
}

#[cfg(test)]
mod template_tests {
    use std::io::MemWriter;
    use std::str;

    use parser::Parser;
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
        Template::render_data(&mut w, &data, &parser);
        assert_eq!(a1, str::from_utf8(w.get_ref()).unwrap());

        w = MemWriter::new();
        data = HashBuilder::new().insert_string("value", s2);
        Template::render_data(&mut w, &data, &parser);
        assert_eq!(a2, str::from_utf8(w.get_ref()).unwrap());
    }

    #[test]
    fn test_not_escape_html() {
        let s2 = "1<2 <b>hello</b>";
        let mut w = MemWriter::new();
        let compiler2 = Compiler::new("{{& value }}");

        let parser = Parser::new(&compiler2.tokens);
        let data = HashBuilder::new().insert_string("value", s2);

        Template::render_data(&mut w, &data, &parser);
        assert_eq!(s2, str::from_utf8(w.get_ref()).unwrap());        
    }

    #[test]
    fn test_render_to_io_stream() {
        let mut w = MemWriter::new();

        let data = HashBuilder::new().insert_string("value1", "The heading");

        let compiler = Compiler::new("<h1>{{ value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);

        Template::render_data(&mut w, &data, &parser);
        assert_eq!("<h1>The heading</h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_string_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("value1", "heading");

        Template::render_data(&mut w, &data, &parser);

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

        Template::render_data(&mut w, &data, &parser);

        assert_eq!(a1.to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_bool_false_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", false);

        Template::render_data(&mut w, &data, &parser);

        assert_eq!("<h1>false</h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_unescaped_node_correct_bool_true_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{& value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", true);

        Template::render_data(&mut w, &data, &parser);

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

        Template::render_data(&mut w, &data, &parser);

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

        Template::render_data(&mut w, &data, &parser);

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

        Template::render_data(&mut w, &data, &parser);

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

        Template::render_data(&mut w, &data, &parser);
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

        Template::render_data(&mut w, &data, &parser);

        assert_eq!(a1.to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_correct_string_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("<h1>{{ value1 }}<h1>");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_string("value1", "heading");

        Template::render_data(&mut w, &data, &parser);

        assert_eq!("<h1>heading<h1>".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_correct_false_bool_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{ value1 }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", false);

        Template::render_data(&mut w, &data, &parser);

        assert_eq!("false".to_string(), String::from_utf8(w.unwrap()).unwrap());
    }

    #[test]
    fn test_value_node_correct_true_bool_data() {
        let mut w = MemWriter::new();
        let compiler = Compiler::new("{{ value1 }}");
        let parser = Parser::new(&compiler.tokens);
        let data = HashBuilder::new().insert_bool("value1", true);

        Template::render_data(&mut w, &data, &parser);

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

        Template::render_data(&mut w, &data, &parser);
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

        Template::render_data(&mut w, &data, &parser);
        assert_eq!(s, String::from_utf8(w.unwrap()).unwrap());
    }
}
