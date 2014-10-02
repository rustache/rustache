#![crate_name = "rustache"]
#![warn(missing_doc)]
// #![deny(warnings)]

#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate serialize;

use std::collections::HashMap;
use std::fmt;
use std::cell::RefCell;

pub use build::{HashBuilder, VecBuilder};
pub use rustache::{render, render_text, render_json_file, render_json_string};

mod rustache;
mod compiler;
mod parser;
mod build;
mod template;

/// Represents the possible types that passed in data may take on
pub enum Data<'a> {
    Strng(String),
    Bool(bool),
    Integer(int),
    Float(f64),
    Vector(Vec<Data<'a>>),
    Hash(HashMap<String, Data<'a>>),
    Lambda(RefCell<|String|: 'a -> String>)
}

impl<'a> PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Strng(ref val0), &Strng(ref val1)) => val0 == val1,
            (&Bool(ref val0), &Bool(ref val1)) => val0 == val1,
            (&Integer(ref val0), &Integer(ref val1)) => val0 == val1,
            (&Float(ref val0), &Float(ref val1)) => val0 == val1,
            (&Vector(ref val0), &Vector(ref val1)) => val0 == val1,
            (&Hash(ref val0), &Hash(ref val1)) => val0 == val1,
            (&Lambda(_), &Lambda(_)) => fail!("Can't compare closures"),
            (_, _) => false
        }
    }
}

impl<'a> fmt::Show for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Strng(ref val)  => write!(f, "String({})", val),
            Bool(val)       => write!(f, "Boolean({})", val),
            Integer(ref val)    => write!(f, "Integer({})", val),
            Float(ref val)      => write!(f, "Float({})", val),
            Vector(ref val) => write!(f, "Vector({})", val),
            Hash(ref val)   => write!(f, "Hash({})", val),
            Lambda(_)       => write!(f, "Lambda(...)") 
        }
    }
}

#[cfg(test)]
mod lib_tests {
    use rustache;
    use std::io::MemWriter;
    use std::io::File;
    use build::HashBuilder;

    #[test]
    fn test_json_parse() {
        let template_path = "test_data/json.template.html";
        let data_path = "test_data/test.json";

        let mut w = MemWriter::new();
        rustache::render_json_file(template_path, data_path, &mut w);

        let mut f = File::create(&Path::new("test_data/json.html"));
        let completed = f.write(w.unwrap().as_slice());
        assert_eq!(completed, Ok(()));
    }

    #[test]
    fn file_end_to_end_test() {
        let path = "test_data/index.template.html";
        let data = HashBuilder::new()
            .insert_hash("people", |builder| {
                builder.insert_vector("information", |builder| {
                    builder
                        .push_string("<tr><td>Fleur</td><td>Dragan</td></tr>")
                        .push_string("<tr><td>Jarrod</td><td>Ruhland</td></tr>")
                        .push_string("<tr><td>Jim</td><td>O'Brien</td></tr>")
                        .push_string("<tr><td>Sean</td><td>Chen</td></tr>")
                    }
                )}
            ).set_partials_path("test_data");
            
        let mut w = MemWriter::new();
        rustache::render(path, &data, &mut w);

        let mut f = File::create(&Path::new("test_data/index.html"));
        let completed = f.write(w.unwrap().as_slice());
        assert_eq!(completed, Ok(()));
    }
}
