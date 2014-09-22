use std::collections::hashmap::HashMap;
use std::io::{File};
use parser::{Node, Tag};

pub struct Template<'a>;

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template
    }

    pub fn render_data<'a>(data: HashMap<&'a str, &'a str>, nodes: &'a Vec<Node>) -> String {
        let mut output = String::new();
        for node in nodes.iter() {
            println!("{}", node);
            if !data.contains_key(&node.val.as_slice()) {
                output = output.append(node.val.as_slice());
            } else {
                output = output.append(data[node.val.as_slice()]);
            }
        }

        output
    }

    pub fn write_to_mem(data: &str, path: &str) {
        let mut file = File::create(&Path::new(path));
        file.write(data.as_bytes());
    }
}