use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use compiler;
use parser;
use rustc_serialize::json::Json;
use rustc_serialize::json::Json::{Boolean, Null, I64, U64, F64, Array, Object};
use rustc_serialize::json::Json::String as JString;
use build::{HashBuilder, VecBuilder};
use template::Template;

use RustacheResult;
use RustacheError::{JsonError, FileError};

/// Defines a `renderable` trait, so that all of our data is renderable
pub trait Render {
    /// `render` function on a `renderable` returns a `reader`
    fn render<W: Write>(&self, template: &str, writer: &mut W) -> RustacheResult<()>;
}

/// Implement the `renderable` trait on the HashBuilder type
impl<'a> Render for HashBuilder<'a> {
    fn render<W: Write>(&self, template: &str, writer: &mut W) -> RustacheResult<()> {
        // Create our nodes
        let tokens = compiler::create_tokens(template);
        let nodes = parser::parse_nodes(&tokens);

        // Render and write out
        Template::new().render_data(writer, self, &nodes)
    }
}


/// Implement the `renderable` trait on the JSON type
impl Render for Json {
    fn render<W: Write>(&self, template: &str, writer: &mut W) -> RustacheResult<()> {
        parse_json(self).render(template, writer)
    }
}

impl Render for Path {
    fn render<W: Write>(&self, template: &str, writer: &mut W) -> RustacheResult<()> {

        return match read_file(self) {
            Ok(text) => {

                let json = match Json::from_str(&text) {
                    Ok(json) => json,
                    Err(err) => return Err(JsonError(format!("Invalid JSON. {}", err)))
                };

                parse_json(&json).render(template, writer)
            },
            Err(err) => {
                Err(FileError(err))
            }
        }
    }
}

impl Render for ToString {
    fn render<W: Write>(&self, template: &str, writer: &mut W) -> RustacheResult<()> {

        let json = match Json::from_str(&self.to_string()) {
            Ok(json) => json,
            Err(err) => return Err(JsonError(format!("Invalid JSON. {}", err)))
        };

        parse_json(&json).render(template, writer)
    }
}

/// Render a template from the given template file
///
/// ```
/// use rustache::HashBuilder;
/// use std::io::Cursor;
///
/// let data = HashBuilder::new().insert("planet", "Earth");
/// let mut rv = Cursor::new(Vec::new());
/// rustache::render_file("test_data/cmdline_test.tmpl", data, &mut rv).unwrap();
/// println!("{}", String::from_utf8(rv.into_inner()).unwrap());
/// ```
pub fn render_file<Re: Render, W: Write>(path: &str, renderable: Re, writer: &mut W) -> RustacheResult<()> {

    return match read_file(&Path::new(path)) {
        Ok(text) => renderable.render(&text[..], writer),
        Err(err) => Err(FileError(err))
    }
}

/// Render the given template string
///
/// ```
/// use rustache::HashBuilder;
/// use std::io::Cursor;
///
/// let data = HashBuilder::new().insert("name", "your name");
/// let mut rv = Cursor::new(Vec::new());
/// rustache::render_text("{{ name }}", data, &mut rv).unwrap();
/// println!("{}", String::from_utf8(rv.into_inner()).unwrap());
/// ```
pub fn render_text<Re: Render, W: Write>(input: &str, renderable: Re, writer: &mut W) -> RustacheResult<()> {
    renderable.render(input, writer)
}

// parses a Rust JSON hash and matches all possible types that may be passed in
// returning a HashBuilder 
fn parse_json(json: &Json) -> HashBuilder {
    let mut data = HashBuilder::new();
    for (k, v) in json.as_object().unwrap().iter() {
        match v {
            &I64(num) => {
                data = data.insert(&k[..], num.to_string());
            }
            &U64(num) => {
                data = data.insert(&k[..], num.to_string());
            },
            &F64(num) => {
                data = data.insert(&k[..], num.to_string());
            },
            &Boolean(val) => {
                data = data.insert(&k[..], val);
            },
            &Array(ref list) => {
                let mut builder = VecBuilder::new();
                for item in list.iter() {
                    builder = match *item {
                        Object(_) => builder.push(parse_json(item)),
                        Array(_) => builder.push(parse_json_vector(item)),
                        JString(_) => builder.push(item.as_string().unwrap()),
                        Boolean(_) => builder.push(item.as_boolean().unwrap()),
                        _ => builder
                    }
                }
                data = data.insert(&k[..], builder);
            },
            &Object(_) => {
                data = data.insert(&k[..], parse_json(v));
            },
            &Null => {},
            &JString(ref text) => {
                data = data.insert(&k[..], &text[..]);
            },
        }
    }

    data
}

// parses a Rust JSON vector and matches all possible types that may be passed in
// returning a VecBuider
fn parse_json_vector(json: &Json) -> VecBuilder {
    let mut data = VecBuilder::new();
    for v in json.as_array().unwrap().iter() {
        match v {
            &I64(num) => {
                data = data.push(num.to_string());
            }
            &U64(num) => {
                data = data.push(num.to_string());
            },
            &F64(num) => {
                data = data.push(num.to_string());
            },
            &Boolean(val) => {
                data = data.push(val);
            },
            &Array(ref list) => {
                let mut builder = VecBuilder::new();
                for item in list.iter() {
                    builder = match *item {
                        Object(_) => builder.push(parse_json(item)),
                        Array(_) => builder.push(parse_json_vector(item)),
                        JString(_) => builder.push(item.as_string().unwrap()),
                        Boolean(_) => builder.push(item.as_boolean().unwrap()),
                        _ => builder
                    }
                }
                data = data.push(builder);
            },
            &Object(_) => {
                data = data.push(parse_json(v));
            },
            &Null => {},
            &JString(ref text) => {
                data = data.push(&text[..]);
            },
        }
    }
    data
}

// Hide from documentation
#[doc(hidden)]
pub fn read_file(path: &Path) -> Result<String, String> {
    let display = path.display();
    let rv: Result<String, String>; //Err(format!("read file failed: {}", display));
    // Open the file path
    let mut file = match File::open(path) {
        Err(why) => { rv = Err(format!("{}: \"{}\"", why, display)); return rv; },
        Ok(file) => { file },
    };

    // Read the file contents into a heap allocated string
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => return { rv = Err(format!("{}", why)); return rv; },
        Ok(_) => { rv = Ok(text); },
    };

    rv
}
