#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::io::{File, MemWriter, stdout};
pub use rustache::Rustache;
pub use template::Template;
pub use parser::Parser;

// Helpers
fn get_template(template_path: &str) -> String {
    let path = Path::new(template_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => fail!("Couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };

    // read file to string 
    let template_str: String = match file.read_to_string() {
        Err(why)   => fail!("Couldn't read {}: {}", display, why.desc),
        Ok(string) =>  string,
    };

    template_str
}


fn render_template_with_data<W: Writer>(writer: &mut W, data: &str) {
    writer.write_str(data).unwrap();
}

// TODO: find out how to get around the limitation of traits in test-function signatures.
// this does not work.
#[test]
// fn should_render_template() {
//     let fake_template:&str = "<div>";
//     render_test_helper(fake_template);
// }


#[test]
fn should_retrieve_file() {
    let path = "src/test_templates/sample.html";
    let expected = String::from_str("<div>");
    let retrieved_template = get_template(path);

    // for testing a stream - not working yet.
    // let passed_template: &str = retrieved_template.as_slice();
    // render_template_with_data(&stream, passed_template);

    assert_eq!(retrieved_template, Ok(expected));
}

mod parser;
mod template;
mod rustache;
