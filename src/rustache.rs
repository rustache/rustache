use std::fs::File;
use std::io::{Cursor, Read};
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
pub trait Render<R: Read> {
    /// `render` function on a `renderable` returns a `reader`
    fn render(&self, template: &str) -> RustacheResult<R>;
}

/// Implement the `renderable` trait on the HashBuilder type
impl<'a> Render<Cursor<Vec<u8>>> for HashBuilder<'a> {
    fn render(&self, template: &str) -> RustacheResult<Cursor<Vec<u8>>> {
        // Create the stream we are going to write to.
        let mut stream = Cursor::new(Vec::new());

        // Create our nodes
        let tokens = compiler::create_tokens(template);
        let nodes = parser::parse_nodes(&tokens);
        
        // Write to our stream.
        try!(Template::new().render_data(&mut stream, self, &nodes));
        
        // Return the stream as a Reader.
        Ok(stream)
    } 
}


/// Implement the `renderable` trait on the JSON type
impl Render<Cursor<Vec<u8>>> for Json {
    fn render(&self, template: &str) -> RustacheResult<Cursor<Vec<u8>>> {
       parse_json(self).render(template)
    }
}

impl Render<Cursor<Vec<u8>>> for Path {
    fn render(&self, template: &str) -> RustacheResult<Cursor<Vec<u8>>> {

        return match read_file(self) {
            Ok(text) => {

                let json = match Json::from_str(&text) {
                    Ok(json) => json,
                    Err(err) => return Err(JsonError(format!("Invalid JSON. {}", err)))
                };

                let hb = parse_json(&json);
                hb.render(template)
            },
            Err(err) => {
                Err(FileError(err))
            }
        }
    }
}

impl Render<Cursor<Vec<u8>>> for String {
    fn render(&self, template: &str) -> RustacheResult<Cursor<Vec<u8>>> {

        let json = match Json::from_str(&self[..]) {
            Ok(json) => json,
            Err(err) => return Err(JsonError(format!("Invalid JSON. {}", err)))
        };

        let hb = parse_json(&json);
        hb.render(template)
    }
}

/// Render a template from the given template file
///
/// ```
/// use rustache::HashBuilder;
///
/// let data = HashBuilder::new().insert("planet", "Earth");
/// let rv = rustache::render_file("test_data/cmdline_test.tmpl", data);
/// println!("{}", String::from_utf8(rv.unwrap().into_inner()).unwrap());
/// ```
pub fn render_file<R: Read, Re: Render<R>>(path: &str, renderable: Re) -> RustacheResult<R> {

    return match read_file(&Path::new(path)) {
        Ok(text) => renderable.render(&text[..]),
        Err(err) => Err(FileError(err))
    }
}

/// Render the given template string
///
/// ```
/// use rustache::HashBuilder;
///
/// let data = HashBuilder::new().insert("name", "your name");
/// let rv = rustache::render_text("{{ name }}", data);
/// println!("{}", String::from_utf8(rv.unwrap().into_inner()).unwrap());
/// ```
pub fn render_text<R: Read, Re: Render<R>>(input: &str, renderable: Re) -> RustacheResult<R> {
    renderable.render(input)
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
