use std::io::Write;
use compiler;
use parser;
use rustc_serialize::json::Json;
use rustc_serialize::json::Json::{Boolean, Null, I64, U64, F64, Array, Object};
use rustc_serialize::json::Json::String as JString;
use build::{HashBuilder, VecBuilder};
use template::Template;

use RustacheResult;
use RustacheError::JsonError;

/// Defines a `renderable` trait, so that all of our data is renderable
pub trait Render {
    /// `render` function on a `renderable` returns a `reader`
    fn render<W: Write>(&self, template: &str, writer: &mut W) -> RustacheResult<()>;
}

/// Implement the `renderable` trait on the `HashBuilder` type
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

impl Render for ToString {
    fn render<W: Write>(&self, template: &str, writer: &mut W) -> RustacheResult<()> {

        let json = match Json::from_str(&self.to_string()) {
            Ok(json) => json,
            Err(err) => return Err(JsonError(format!("Invalid JSON. {}", err))),
        };

        parse_json(&json).render(template, writer)
    }
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
            }
            &F64(num) => {
                data = data.insert(&k[..], num.to_string());
            }
            &Boolean(val) => {
                data = data.insert(&k[..], val);
            }
            &Array(ref list) => {
                let mut builder = VecBuilder::new();
                for item in list.iter() {
                    builder = match *item {
                        Object(_) => builder.push(parse_json(item)),
                        Array(_) => builder.push(parse_json_vector(item)),
                        JString(_) => builder.push(item.as_string().unwrap()),
                        Boolean(_) => builder.push(item.as_boolean().unwrap()),
                        _ => builder,
                    }
                }
                data = data.insert(&k[..], builder);
            }
            &Object(_) => {
                data = data.insert(&k[..], parse_json(v));
            }
            &Null => {}
            &JString(ref text) => {
                data = data.insert(&k[..], &text[..]);
            }
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
            }
            &F64(num) => {
                data = data.push(num.to_string());
            }
            &Boolean(val) => {
                data = data.push(val);
            }
            &Array(ref list) => {
                let mut builder = VecBuilder::new();
                for item in list.iter() {
                    builder = match *item {
                        Object(_) => builder.push(parse_json(item)),
                        Array(_) => builder.push(parse_json_vector(item)),
                        JString(_) => builder.push(item.as_string().unwrap()),
                        Boolean(_) => builder.push(item.as_boolean().unwrap()),
                        _ => builder,
                    }
                }
                data = data.push(builder);
            }
            &Object(_) => {
                data = data.push(parse_json(v));
            }
            &Null => {}
            &JString(ref text) => {
                data = data.push(&text[..]);
            }
        }
    }
    data
}
