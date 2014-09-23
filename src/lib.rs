#![crate_name = "rustache"]

use std::collections::HashMap;
use std::fmt;

pub use build::Builder;
pub use template::Template;
pub use parser::{Parser, Node};

#[deriving(Show, PartialEq)]
pub enum Data<'a> {
    Static(String),
    Bool(bool),
    Vector(Vec<Data<'a>>),
    Map(HashMap<String, Data<'a>>)
}

// #[test]
// fn basic_end_to_end_test() {
    // use std::collections::hashmap::HashMap;
    // use std::io::MemWriter;
    // use std::str;

    // let mut mem_wr = MemWriter::new();
    // let mut data_map: HashMap<&str, &str> = HashMap::new();

    // data_map.insert("value1", "Bob");
    // data_map.insert("value2", "Tom");
    // data_map.insert("value3", "Joe");

    // let in_path = "examples/template_files/basic_sample.html";
    // let out_path = "examples/template_files/basic_output.html";
    // let in_data = Parser::read_template(in_path);
    // let tags = Parser::tag_lines(in_data);
    // let tokens = Parser::create_token_map_from_tags(&tags);
    // let data = Build::create_data_map(tokens, data_map);

    // // write to memwriter stream
    // Template::render_data(&mut mem_wr, data, &tags);

    // // unwrap bytes
    // let output_bytes = mem_wr.unwrap();

    // // bytes to string
    // let output = str::from_utf8(output_bytes.as_slice()).unwrap().to_string();

    // let mut expected: String = String::new();
    // expected = expected.append("<html><body><div><span>Bob</span></div><div><span>Tom</span></div><div><b>Joe</b><a></a></div></body></html>");
    // assert_eq!(output, expected);
// }

mod parser;
mod build;
mod template;
