use std::collections::hashmap::HashMap;
use parser::{Parser, Node, Value, Static, Unescaped};
use build::{HashBuilder};
use super::{Data, Str, Bool, Vector, Hash};
use std::io::stdio::stdout;
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
                _    => { rv.push_char(c); }
            }
        }
        rv
    }

    fn handle_unescaped_node<'a, W: Writer>(key: &String, data: &Data, writer: &mut W) {
        let mut tmp: String = String::new();
        match *data {

            Str(ref val) => {
                tmp = tmp + *val;
            }
            Bool(val) => {
                if val {
                    tmp.push_str("true");
                } else {
                    tmp.push_str("false");
                }
            }
            Vector(_) => {
                fail!("expecting text, found vector data");
            }
            Hash(_) => {
                fail!("expecting text, found hash data");
            }
        }

        if tmp.len() != 0 {
            writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
        }        
    }

    fn handle_value_node<'a, W: Writer>(key: &String, data: &Data, writer: &mut W) {
        let mut tmp: String = String::new();
        match *data {

            Str(ref val) => {
                tmp = *Template::escape_html(&(*val.as_slice()));
            }
            Bool(val) => {
                if val {
                    tmp.push_str("true");
                } else {
                    tmp.push_str("false");
                }
            }
            Vector(_) => {
                fail!("expecting text, found vector data");
            }
            Hash(_) => {
                fail!("expecting text, found hash data");
            }
        }

        if tmp.len() != 0 {
            writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
        }        
    }

    pub fn render_data<'a, W: Writer>(writer: &mut W,  
                                      datastore: &HashBuilder, 
                                      parser: &Parser) {
        let mut tmp: String = String::new();
        for node in parser.nodes.iter() {
            tmp.truncate(0);
            match *node {
                Unescaped(key)  => {
                    let tmp = key.to_string();
                    if datastore.data.contains_key(&tmp) {
                        let ref val = datastore.data[tmp];
                        Template::handle_unescaped_node(&tmp, val, writer);
                    }
                }
                Value(key) => {
                    let tmp = key.to_string();
                    if datastore.data.contains_key(&tmp) {
                        let ref val = datastore.data[tmp];
                        Template::handle_value_node(&tmp, val, writer);
                    }
                }

                Static(ref key) => {
                     tmp.push_str(key.as_slice());
                }
                _ => continue
            }
            writer.write_str(tmp.as_slice()).ok().expect("write failed in render");
        }
    }
}



#[cfg(test)]
mod template_tests {
     use std::collections::hashmap::HashMap;
     use std::io::MemWriter;

     use parser::Parser;
     use template::Template;
     use compiler::Compiler;
     use build::HashBuilder;
     use std::str;

    #[test]
    fn test_escape_html() {
        let mut data_map: HashMap<String, String> = HashMap::new();
        let s1 = "a < b > c & d \"spam\"\'";
        let a1 = "a &lt; b &gt; c &amp; d &quot;spam&quot;'";
        let s2 = "1<2 <b>hello</b>";
        let a2 = "1&lt;2 &lt;b&gt;hello&lt;/b&gt;";

        let mut w = MemWriter::new();
        let mut compiler = Compiler::new("{{ value }}");
        let mut parser = Parser::new(&compiler.tokens);
        let mut data = HashBuilder::new().insert_string("value", s1);
        Template::render_data(&mut w, &data, &parser);
        assert_eq!(a1, str::from_utf8(w.get_ref()).unwrap());

        w = MemWriter::new();
        data = HashBuilder::new().insert_string("value", s2);
        Template::render_data(&mut w, &data, &parser);
        assert_eq!(a2, str::from_utf8(w.get_ref()).unwrap());

        w = MemWriter::new();
        let compiler2 = Compiler::new("{{& value }}");
        parser = Parser::new(&compiler2.tokens);
        data = HashBuilder::new().insert_string("value", s2);
        Template::render_data(&mut w, &data, &parser);
        assert_eq!(s2, str::from_utf8(w.get_ref()).unwrap());
    }

    #[test]
    fn test_render_to_io_stream() {
        let mut w = MemWriter::new();
        let mut data_map: HashMap<String, String> = HashMap::new();

        let data = HashBuilder::new().insert_string("value1", "The heading");

        let compiler = Compiler::new("<h1>{{ value1 }}</h1>");
        let parser = Parser::new(&compiler.tokens);

        Template::render_data(&mut w, &data, &parser);
        assert_eq!("<h1>The heading</h1>".to_string(), str::from_utf8_owned(w.unwrap()).unwrap());
    }
}
