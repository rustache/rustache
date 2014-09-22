use std::collections::hashmap::HashMap;
use std::io::{File};
use parser::{Node};

pub struct Template<'a>;

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template
    }

    // pub fn render_data<'a, W: Writer>(writer: &mut W,  data: HashMap<&'a str, &'a str>, nodes: &'a Vec<Node>) {
    //     let mut output = String::new();
    //     for node in nodes.iter() {
    //         if !data.contains_key(&node.val.as_slice()) {
    //             writer.write_str(node.val.as_slice());
    //         } else {
    //             writer.write_str(data[node.val.as_slice()]);
    //         }
    //     }
    // }
}