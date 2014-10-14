#![license = "MIT"]

#![warn(missing_doc)]
#![warn(warnings)]

#![feature(phase)]
//! The main crate for the Rustache library.
//!
//! Rustache is a flexible template engine for Rust.

// StdLib dependencies
#[phase(plugin)] 
extern crate regex_macros;
extern crate regex;
extern crate serialize;
extern crate memstream;

use std::fmt;
use std::cell::RefCell;
use std::collections::HashMap;

use template::TemplateError;

pub use build::{HashBuilder, VecBuilder};
pub use rustache::{render_file, render_text, Render};

pub type RustacheResult<T> = Result<T, RustacheError>;

/// Enum to handle errors from the Rustache library.
pub enum RustacheError {
    //ParserErrorType(ParserError),
    //CompilerErrorType(CompilerError),
    /// Error parsing JSON data
    JsonError(String),
    /// Error opening or reading a file
    FileError(String),
    /// Generic enum value for any errors from the template module.
    TemplateErrorType(TemplateError)
}

impl fmt::Show for RustacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &JsonError(ref val) => write!(f, "JsonError: {}", val),
            &FileError(ref val) => write!(f, "FileError: {}", val),
            &TemplateErrorType(ref val) => write!(f, "{}", val),
        }
    }
}

// Represents the possible types that passed in data may take on
#[doc(hidden)]
pub enum Data<'a> {
    Strng(String),
    Bool(bool),
    Integer(int),
    Float(f64),
    Vector(Vec<Data<'a>>),
    Hash(HashMap<String, Data<'a>>),
    Lambda(RefCell<|String|: 'a -> String>)
}

// Implementing custom PartialEq for Data
impl<'a> PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Strng(ref val0), &Strng(ref val1)) => val0 == val1,
            (&Bool(ref val0), &Bool(ref val1)) => val0 == val1,
            (&Integer(ref val0), &Integer(ref val1)) => val0 == val1,
            (&Float(ref val0), &Float(ref val1)) => val0 == val1,
            (&Vector(ref val0), &Vector(ref val1)) => val0 == val1,
            (&Hash(ref val0), &Hash(ref val1)) => val0 == val1,
            (&Lambda(_), &Lambda(_)) => fail!("Can't compare closures"),
            (_, _) => false
        }
    }
}

// Implementing custom Show for Data
impl<'a> fmt::Show for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Strng(ref val)   => write!(f, "Strng({})", val),
            Bool(val)        => write!(f, "Boolean({})", val),
            Integer(ref val) => write!(f, "Integer({})", val),
            Float(ref val)   => write!(f, "Float({})", val),
            Vector(ref val)  => write!(f, "Vector({})", val),
            Hash(ref val)    => write!(f, "Hash({})", val),
            Lambda(_)        => write!(f, "Lambda(...)") 
        }
    }
}

// Internal Modules
mod rustache;
mod compiler;
mod parser;
mod build;
mod template;
