#![feature(phase)]
#[phase(plugin)]

pub use rustache::Rustache;
pub use template::Template;
pub use parser::Parser;

mod parser;
mod template;
mod rustache;
