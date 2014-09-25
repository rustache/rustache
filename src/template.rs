use std::collections::hashmap::HashMap;
use parser::{Node, Value, Static, Unescaped};

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

    pub fn render_data<'a, W: Writer>(writer: &mut W,  
                                      data: &HashMap<String, String>, 
                                      nodes: &Vec<Node>) {
        let mut tmp: String = String::new();
        for node in nodes.iter() {
            tmp.truncate(0);
            match *node {
                Unescaped(ref text)  => {
                    if data.contains_key(&text.to_string()) {
                        let ref val = data[text.to_string()];
                        tmp.push_str(val.as_slice());
                    }
                }
                Value(ref text) => {
                    if data.contains_key(&text.to_string()) {
                        let val = data[text.to_string()].as_slice();
                        tmp = *Template::escape_html(val);
                    }
                }
                Static(ref text) => {
                    tmp.push_str(text.as_slice());
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

    use parser::Parser::tokenize;
    use template::Template;
    use std::str;

    #[test]
    fn test_escape_html() {
        let mut data_map: HashMap<String, String> = HashMap::new();
        let s1 = "a < b > c & d \"spam\"\'";
        let a1 = "a &lt; b &gt; c &amp; d &quot;spam&quot;'";
        let s2 = "1<2 <b>hello</b>";
        let a2 = "1&lt;2 &lt;b&gt;hello&lt;/b&gt;";
        let mut tags = Parser::tokenize("{{ value }}");

        let mut w = MemWriter::new();
        data_map.insert("value".to_string(), s1.to_string());
        Template::render_data(&mut w, &data_map, &tags);
        assert_eq!(a1, str::from_utf8(w.get_ref()).unwrap());

        w = MemWriter::new();
        data_map.insert("value".to_string(), s2.to_string());
        Template::render_data(&mut w, &data_map, &tags);
        assert_eq!(a2, str::from_utf8(w.get_ref()).unwrap());

        w = MemWriter::new();
        tags = Parser::tokenize("{{& value }}");
        data_map.insert("value".to_string(), s2.to_string());
        Template::render_data(&mut w, &data_map, &tags);
        assert_eq!(s2, str::from_utf8(w.get_ref()).unwrap());
    }

    #[test]
    fn test_render_to_io_stream() {
        let mut w = MemWriter::new();
        let mut data_map: HashMap<String, String> = HashMap::new();
        let tags = Parser::tokenize("<h1>{{ value1 }}</h1>");

        data_map.insert("value1".to_string(), "The heading".to_string());

        Template::render_data(&mut w, &data_map, &tags);
        assert_eq!("<h1>The heading</h1>".to_string(), str::from_utf8_owned(w.unwrap()).unwrap());
    }
}
