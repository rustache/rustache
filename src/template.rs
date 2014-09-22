use std::collections::hashmap::HashMap;
use std::io::{File};
use parser::{Node, Value, Static};

pub struct Template<'a>;

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template
    }

    pub fn render_data<'a, W: Writer>(writer: &mut W,  data: HashMap<&'a str, &'a str>, nodes: Vec<Node>) {
        for node in nodes.iter() {
            match *node {
                Value(text)  => {
                    if !data.contains_key(&text.as_slice()) {
                        writer.write_str(text.as_slice());
                    } else {
                        writer.write_str(data[text.as_slice()]);
                    }
                }
                Static(text) => {
                    writer.write_str(text.as_slice())
                }
            }

/*            if !data.contains_key(&node.val.as_slice()) {
                writer.write_str(node.val.as_slice());
            } else {
                writer.write_str(data[node.val.as_slice()]);
            }*/
        }
    }
}
