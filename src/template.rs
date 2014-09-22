use std::collections::hashmap::HashMap;
use std::io::{BufferedWriter, File};
use parser::{Node, Tag};

pub struct Template<'a>;

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template
    }

    pub fn render_data<'a, W: Writer>(writer: &mut W,  data: HashMap<&'a str, &'a str>, nodes: &'a Vec<Node>) {
        let mut output = String::new();
        for node in nodes.iter() {
            if !data.contains_key(&node.val.as_slice()) {
                writer.write_str(node.val.as_slice());
            } else {
                writer.write_str(data[node.val.as_slice()]);
            }
        }
    }

    pub fn write_to_mem(data: &str, path: &str) {
        let mut file = File::create(&Path::new(path));
        file.write(data.as_bytes());
    }
}