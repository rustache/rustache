#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub use build::Build;
pub use template::Template;
pub use parser::Parser;


fn main() {
   let mut data_map: HashMap<&str, &str> = HashMap::new();
    data_map.insert("{{ value1 }}", "Bob");
    data_map.insert("{{ value2 }}", "Tom");
    data_map.insert("{{ value3 }}", "Joe");

    let path = "src/test_templates/sample.html";
    let file_stuff = read_template(path);
    let tags = tag_lines(file_stuff);
    let tokens = create_token_map_from_tags(&tags);
    let data = create_data_map(tokens, data_map);
    let output = render_data(data, &tags);

    write_to_file(output.as_slice());
}

mod parser;
mod build;
mod template;
