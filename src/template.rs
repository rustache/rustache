use std::collections::hashmap::HashMap;
use std::io::{IoError, stdout};
use parser::{Node, Value, Static};

pub struct Template<'a>;

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template
    }


    pub fn render_data<'a, W: Writer>(writer: &mut W,  
                                      data: HashMap<String, String>, 
                                      nodes: &'a Vec<Node>) {
        let mut tmp: &str = "";
        for node in nodes.iter() {
            tmp = "";
            match *node {
                Value(ref text)  => {
                    stdout().write_str(text.as_slice()).ok().expect("value");
                    if data.contains_key(text) {
                        tmp = data[text.to_string()].as_slice();
                   }
                }
                Static(ref text) => {
                    tmp = text.as_slice()
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
    use std::io::{TempDir, File, BufferedWriter, BufferedReader};
    use std::io::{stdout, MemWriter};

    use parser::Parser;
    use template::Template;
    use std::str;

    #[test]
    fn test_write_to_console() {
        // let mut buf = vec![0, ..256];
        let mut w = MemWriter::new();
        let mut data_map: HashMap<String, String> = HashMap::new();
        let tags = Parser::tokenize_line("<h1>{{ value1 }}</h1>");

        data_map.insert("value1".to_string(), "The heading".to_string());

        Template::render_data(&mut w, data_map, &tags);
        assert_eq!("<h1>The heading</h1>".to_string(), str::from_utf8_owned(w.unwrap()).unwrap());
    }
}
