#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

pub use build::Build;
pub use template::Template;
pub use parser::Parser;

mod parser;
mod build;
mod template;
