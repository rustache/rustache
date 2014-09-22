#![crate_name = "rustache"]

#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub use build::Build;
pub use template::Template;
pub use parser::Parser;

#[test]
fn basic_end_to_end_test() {
    use std::collections::hashmap::HashMap;

    let mut data_map: HashMap<&str, &str> = HashMap::new();
    data_map.insert("value1", "Bob");
    data_map.insert("value2", "Tom");
    data_map.insert("value3", "Joe");

    let in_path = "examples/template_files/basic_sample.html";
    let out_path = "examples/template_files/basic_output.html";
    let in_data = Parser::read_template(in_path);
    let tags = Parser::tag_lines(in_data);
    let tokens = Parser::create_token_map_from_tags(&tags);
    let data = Build::create_data_map(tokens, data_map);
    let output = Template::render_data(data, &tags);

    Template::write_to_mem(output.as_slice(), out_path);
    // let mut expected: String = String::new();
    // expected = expected.append("<html><body><div>Bob</div><div>Tom</div><div>Joe</div></body></html>");
    assert!(false);
}

mod parser;
mod build;
mod template;
