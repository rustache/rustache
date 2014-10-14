use std::io::File;
use compiler;
use parser;
use memstream::MemStream;
use serialize::json::{Json, Boolean, Null, I64, U64, F64, String, List, Object};
use build::{HashBuilder, VecBuilder};
use template::Template;

use RustacheResult;

/// Defines a `renderable` trait, so that all of our data is renderable
pub trait Render<R: Reader> {
    /// `render` function on a `renderable` returns a `reader`
    fn render(&self, template: &str) -> RustacheResult<R>;
}

/// Implement the `renderable` trait on the HashBuilder type
impl<'a> Render<MemStream> for HashBuilder<'a> {
    fn render(&self, template: &str) -> RustacheResult<MemStream> {
        // Create the stream we are going to write to.
        let mut stream = MemStream::new();

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
impl Render<MemStream> for Json {
    fn render(&self, template: &str) -> RustacheResult<MemStream> {
       parse_json(self).render(template)
    }
}

/// Render a template from the given file
///
/// ```ignore
/// use rustache;
///
/// let data = json::from_str(r#"{"name": "Bob"}"#);
/// rustache::render_file("path/to/template.html", data);
/// ```
pub fn render_file<R: Reader, Re: Render<R>>(path: &str, renderable: Re) -> RustacheResult<R> {
    renderable.render(File::open(&Path::new(path)).read_to_string().as_slice()[0].as_slice())
}

/// Render the given template string
///
/// ```ignore
/// use rustache;
/// 
/// let data = HashBuilder::new()
///     .insert_string("name", "Bob");
///
/// rustache::render_text("{{ name }}", &data);
/// ```
pub fn render_text<R: Reader, Re: Render<R>>(input: &str, renderable: Re) -> RustacheResult<R> {
    renderable.render(input)
}

// parses a Rust JSON hash and matches all possible types that may be passed in
// returning a HashBuilder 
fn parse_json(json: &Json) -> HashBuilder {
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
                        builder = match *item {
                            Object(_) => builder.push_hash(|_| {
                                parse_json(item)
                            }),
                            List(_) => builder.push_vector(|_| {
                                parse_json_vector(item)
                            }),
                            String(_) => builder.push_string(item.as_string().unwrap()),
                            Boolean(_) => builder.push_bool(item.as_boolean().unwrap()),
                            _ => builder
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

// parses a Rust JSON vector and matches all possible types that may be passed in
// returning a VecBuider
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
                        builder = match *item {
                            Object(_) => builder.push_hash(|_| {
                                parse_json(item)
                            }),
                            List(_) => builder.push_vector(|_| {
                                parse_json_vector(item)
                            }),
                            String(_) => builder.push_string(item.as_string().unwrap()),
                            Boolean(_) => builder.push_bool(item.as_boolean().unwrap()),
                            _ => builder
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

// Hide from documentation
#[doc(hidden)]
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
