use std::io::File;
use compiler;
use parser;
use serialize::{json};
use serialize::json::{Json, Boolean, Null, I64, U64, F64, String, List, Object};
use build::{HashBuilder, VecBuilder};
use template::Template;

/// Render a text file from a HashBuilder data source to the specified writer
///
/// ```rust
/// extern crate rustache;
///
/// use rustache;
/// use rustache::HashBuilder;
/// use std::MemWriter;
///
/// let w = MemWriter::new();
/// let data = HashBuilder::new()
///     .insert_string("name", "Bob the Builder")
///     .build();
///
/// rustache::render("templates/index.tpl.html", &data, &mut w);
/// ```
pub fn render<'a, W: Writer>(path: &str, data: &HashBuilder, writer: &mut W) {
    let file = read_file(Path::new(path));
    let tokens = compiler::create_tokens(file.as_slice());
    let nodes = parser::parse_nodes(&tokens);
    Template::new().render_data(writer, data, &nodes);
}

/// Render a string from a HashBuilder data source to the specified writer
///
/// ```rust
/// extern crate rustache;
///
/// use rustache;
/// use rustache::HashBuilder;
/// use std::MemWriter;
///
/// let w = MemWriter::new();
/// let data = HashBuilder::new()
///     .insert_string("name", "Bob the Builder")
///     .build();
///
/// rustache::render_text("{{ name }}", &data, &mut w);
/// ```
pub fn render_text<'a, W: Writer>(input: &'a str, data: &HashBuilder, writer: &mut W) {
    let tokens = compiler::create_tokens(input);
    let nodes = parser::parse_nodes(&tokens);
    Template::new().render_data(writer, data, &nodes);
}

/// Render a template  from a HashBuilder data source to the specified writer
///
/// ```rust
/// extern crate rustache;
///
/// use rustache;
/// use rustache::HashBuilder;
/// use std::MemWriter;
///
/// let w = MemWriter::new();
/// let data = HashBuilder::new()
///     .insert_string("name", "Bob the Builder")
///     .build();
///
/// rustache::render_text("{{ name }}", &data, &mut w);
/// ```
pub fn render_json<W: Writer>(template: &str, json: Json, writer: &mut W) {
    let data = parse_json(&json);
    render(template, &data, writer);
}

pub fn render_json_string<W: Writer>(template: &str, data: &str, writer: &mut W) {
    let json = match json::from_str(data) {
        Ok(json) => json,
        Err(err) => fail!("Invalid JSON. {}", err)
    };
    
    render_json(template, json, writer);
}


pub fn render_json_file<W: Writer>(template: &str, data: &str, writer: &mut W) {
    let data_string = read_file(Path::new(data));

    let json = match json::from_str(data_string.as_slice()) {
        Ok(json) => json,
        Err(err) => fail!("Invalid JSON. {}", err)
    };
    
    render_json(template, json, writer);
}

fn parse_json(json: &Json) -> HashBuilder{
    let mut data = HashBuilder::new();
    for (k, v) in json.as_object().unwrap().iter() {
        match v {
            &I64(num) => {
                data = data.insert_string(k.as_slice(), num.to_string());
            }
            &U64(num) => {
                data = data.insert_string(k.as_slice(), num.to_string());
            },
            &F64(num) => {
                data = data.insert_string(k.as_slice(), num.to_string());
            },
            &Boolean(val) => {
                data = data.insert_bool(k.as_slice(), val);
            },
            &List(ref list) => {
                data = data.insert_vector(k.as_slice(), |mut builder| {
                    for item in list.iter() {
                        if item.is_object() {
                            builder = builder.push_hash(|_| {
                                parse_json(item)
                            });
                        } else if item.is_list() {
                            builder = builder.push_vector(|_| {
                                parse_json_vector(item)
                            });
                        } else if item.is_string() {
                            builder = builder.push_string(item.as_string().unwrap());
                        } else if item.is_boolean() {
                            builder = builder.push_bool(item.as_boolean().unwrap());
                        }
                    }
                    builder
                });
            },
            &Object(ref obj) => {
                data = data.insert_hash(k.as_slice(), |_| {
                    parse_json(v)
                });
            },
            &Null => {},
            &String(ref text) => {
                data = data.insert_string(k.as_slice(), text.as_slice());
            },
        }
    }

    data
}

fn parse_json_vector(json: &Json) -> VecBuilder {
    let mut data = VecBuilder::new();
    for v in json.as_list().unwrap().iter() {
        match v {
            &I64(num) => {
                data = data.push_string(num.to_string());
            }
            &U64(num) => {
                data = data.push_string(num.to_string());
            },
            &F64(num) => {
                data = data.push_string(num.to_string());
            },
            &Boolean(val) => {
                data = data.push_bool(val);
            },
            &List(ref list) => {
                data = data.push_vector(|mut builder| {
                    for item in list.iter() {
                        if item.is_object() {
                            builder = builder.push_hash(|_| {
                                parse_json(item)
                            });
                        } else if item.is_list() {
                            builder = builder.push_vector(|_| {
                                parse_json_vector(item)
                            });
                        } else if item.is_string() {
                            builder = builder.push_string(item.as_string().unwrap());
                        } else if item.is_boolean() {
                            builder = builder.push_bool(item.as_boolean().unwrap());
                        }
                    }
                    builder
                });
            },
            &Object(ref obj) => {
                data = data.push_hash(|_| {
                    parse_json(v)
                });
            },
            &Null => {},
            &String(ref text) => {
                data = data.push_string(text.as_slice());
            },
        }
    }
    data
}

pub fn read_file(path: Path) -> String {
    // Open the file path
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => fail!("{} {}",display ,why.desc),
        Ok(file) => file,
    };

    // Read the file contents into a heap allocated string
    let contents = match file.read_to_string() {
        Err(why) => fail!("{}", why.desc),
        Ok(text) => text,
    };

    contents
}
