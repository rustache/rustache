#![crate_name = "rustache"]

use std::collections::HashMap;
use std::fmt;
use std::io::File;

pub use build::{HashBuilder, VecBuilder};
pub use template::Template;
pub use parser::{Parser, Node};

pub enum Data<'a> {
    Str(String),
    Bool(bool),
    Vector(Vec<Data<'a>>),
    Hash(HashMap<String, Data<'a>>)
}

impl<'a> PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Str(ref val0), &Str(ref val1)) => val0 == val1,
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
            Str(ref val) => write!(f, "String({})", val),
            Bool(val)    => write!(f, "Boolean({})", val),
            Vector(ref val) => write!(f, "Vector({})", val),
            Hash(ref val)    => write!(f, "Hash({})", val) 
        }
    }
}

pub struct Read;

impl Read {
    pub fn read_file(template_path: &str) -> String {
        let path = Path::new(template_path);
        // Open the file path
        let mut file = match File::open(&path) {
            Err(why) => fail!("{}", why.desc),
            Ok(file) => file,
        };

        // Read the file contents into a string
        let contents = match file.read_to_string() {
            Err(why) => fail!("{}", why.desc),
            Ok(text) => text,
        };

        contents
    }
}

mod compiler;
mod parser;
mod build;
mod template;
