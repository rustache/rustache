use std::collections::hashmap::HashMap;
use std::io::{File};
use parser::{Node};

pub struct Template;

impl Template {
    pub fn render_data<'a>(data: HashMap<&'a str, &'a str>, nodes: &'a Vec<Node<'a>>) -> String{
        let mut output = String::new();
        for node in nodes.iter() {
            // match node.node_type {
                // Text  => output = output.append(node.val.as_slice()),
                // Value => output = output.append(data[node.val.as_slice()])
            // };
            println!("{}", node.val);
        }

        output
    }

    pub fn write_to_file(data: &str) {
        let mut file = File::create(&Path::new("src/test_templates/sample_test.html"));
        file.write(data.as_bytes());
    }
}