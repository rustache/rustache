#![crate_name = "rustache"]

extern crate serialize;

use std::collections::HashMap;
use std::fmt;
use std::io::File;
use std::cell::RefCell;
use serialize::{json};

use template::Template;
use compiler::Compiler;
use parser::{Parser};

pub use build::{HashBuilder, VecBuilder};

mod compiler;
mod parser;
mod build;
mod template;

pub struct Rustache;

impl Rustache {
    pub fn new () -> Rustache{
        Rustache
    }


    fn render<'a, W: Writer>(&self, path: &str, data: &HashBuilder, writer: &mut W) {
        let file = Read::read_file(Path::new(path));
        let compiler = Compiler::new(file.as_slice());
        let parser = Parser::new(&compiler.tokens);
        Template::new().render_data(writer, data, &parser);
    }

    fn render_text<'a, W: Writer>(&self, input: &'a str, data: &HashBuilder, writer: &mut W) {
        let compiler = Compiler::new(input);
        let parser = Parser::new(&compiler.tokens);
        Template::new().render_data(writer, data, &parser);
    }

    fn render_json<'a, W: Writer>(&self, template_path: &str, data_path: &str, writer: &mut W) {
        let data_string = Read::read_file(Path::new(data_path));

        let data = match json::from_str(data_string.as_slice()) {
            Ok(json) => json,
            Err(err) => fail!("Invalid JSON. {}", err)
        };

        // decode JSON @ data_path into rust object
            // build a HashBuilder from parsing json

        // self.render()
    }
}

struct Read;

impl Read {
    pub fn read_file(path: Path) -> String {
        // Open the file path
        let dis = path.display();
        let mut file = match File::open(&path) {
            Err(why) => fail!("{} {}",dis ,why.desc),
            Ok(file) => file,
        };

        // Read the file contents into a heap allocated string
        let contents = match file.read_to_string() {
            Err(why) => fail!("{}", why.desc),
            Ok(text) => text,
        };

        contents
    }
}

/// Represents the possible types that passed in data may take on
pub enum Data<'a> {
    Strng(String),
    Bool(bool),
    Vector(Vec<Data<'a>>),
    Hash(HashMap<String, Data<'a>>),
    Func(RefCell<|String|: 'a -> String>)
}

impl<'a> PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Strng(ref val0), &Strng(ref val1)) => val0 == val1,
            (&Bool(ref val0), &Bool(ref val1)) => val0 == val1,
            (&Vector(ref val0), &Vector(ref val1)) => val0 == val1,
            (&Hash(ref val0), &Hash(ref val1)) => val0 == val1,
            (&Func(_), &Func(_)) => fail!("Can't compare closures"),
            (_, _) => false
        }
    }
}

impl<'a> fmt::Show for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Strng(ref val)  => write!(f, "String({})", val),
            Bool(val)       => write!(f, "Boolean({})", val),
            Vector(ref val) => write!(f, "Vector({})", val),
            Hash(ref val)   => write!(f, "Hash({})", val),
            Func(_)         => write!(f, "Func(...)") 
        }
    }
}

#[cfg(test)]
mod lib_tests {
    use std::io::MemWriter;
    use std::io::File;
    use build::HashBuilder;
    use super::Rustache;

    #[test]
    fn test_json_parse() {
        let template_path = "test_data/index.template.html";
        let data_path = "test_data/test.json";

        let mut w = MemWriter::new();
        let r = Rustache::new();
        r.render_json(template_path, data_path, &mut w);

        assert!(false);

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
        let r = Rustache::new();
        r.render(path, &data, &mut w);

        let mut f = File::create(&Path::new("test_data/index.html"));
        let completed = f.write(w.unwrap().as_slice());
        assert_eq!(completed, Ok(()));
    }
}

