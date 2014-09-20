#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub use rustache::Rustache;
pub use template::Template;
pub use parser::Parser;

mod parser;
mod template;
mod rustache;
