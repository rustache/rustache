#![crate_name = "rustache"]

use std::collections::HashMap;
use std::fmt;

pub use build::{HashBuilder, VecBuilder};
pub use template::Template;
pub use parser::{Parser, Node};

pub enum Data<'a> {
    Static(String),
    Bool(bool),
    Vector(Vec<Data<'a>>),
    Hash(HashMap<String, Data<'a>>)
}

impl<'a> PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Static(ref val0), &Static(ref val1)) => val0 == val1,
            (&Bool(ref val0), &Bool(ref val1)) => val0 == val1,
            (&Vector(ref val0), &Vector(ref val1)) => val0 == val1,
            (&Hash(ref val0), &Hash(ref val1)) => val0 == val1,
            (_, _) => false
        }
    }
}

impl<'a> fmt::Show for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Static(ref val) => write!(f, "String({})", val),
            Bool(val)    => write!(f, "Boolean({})", val),
            Vector(ref val) => write!(f, "Vector({})", val),
            Hash(ref val)    => write!(f, "Hash({})", val) 
        }
    }
}

/*#[test]
fn basic_end_to_end_test() {
    use std::collections::hashmap::HashMap;
    use std::io::MemWriter;
    use std::str;

    let mut mem_wr = MemWriter::new();
    // we'll want data_map to eventually look like: 
    // let mut data_map: HashMap<&str, Data<'a>>
    let mut data_map: HashMap<&str, &str> = HashMap::new();

    data_map.insert("value1", "Bob");
    data_map.insert("value2", "Tom");
    data_map.insert("value3", "Joe");

    let in_path = "examples/template_files/basic_sample.html";
    let out_path = "examples/template_files/basic_output.html";
    let in_data = Parser::read_template(in_path);
    let tags = Parser::tokenize(in_data.as_slice());
    let tokens = Parser::create_map_from_tokens(tags.clone());
    // let data = Builder::normalize_data_map(tokens, data_map);

    // write to memwriter stream
    Template::render_data(&mut mem_wr, &data, &tags);

    // unwrap bytes
    let output_bytes = mem_wr.unwrap();

    // bytes to string
    let output = str::from_utf8(output_bytes.as_slice()).unwrap().to_string();

    let mut expected: String = String::new();
    expected = expected.append("<html><body><div><span>Bob</span></div><div><span>Tom</span></div><div><b>Joe</b><a></a></div></body></html>");
    assert_eq!(output, expected);
}*/

mod parser;
mod build;
mod template;
