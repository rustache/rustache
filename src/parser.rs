//! A simple parser for parsing rustache files.
//!
//! Can parse parse opening and closing rustaches and text nodes.

#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
use std::collections::hashmap::HashMap;

// Parse an HTML template and returns a HashMap of the tags 
pub fn parse(source: Vec<Token>) -> HashMap {
    let tag_map: HashMap<String, Vec<Token>> = HashMap::new();
    for token in source.iter() {
        tag_map.insert(token);
    }
    tag_map
}

// Parse a single string tag
fn parse_string_tag(input: &str, token: &Token) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let tokens = Parser::find_token_matches(input);
    for token in tokens {
        let (pos, name, tag_type) = token;
        Token::new(pos, name, tag_type);
    }
    result
}

struct Token<'a> {
    pos: uint,
    name: &'a str,
    tag_type: &'a Tag 
}

impl Token {
    fn new(pos: uint, name: &str, tag_type: Tag) -> Token {
        Token {
            pos: pos,
            name: name,
            tag_type: tag_type
        }
    }
}


 pub enum Tag<'a> {
    Unescaped,
    Variable,
    Truthy,
    Falsy, 
    List,
    Lambda,
    Inverted,
    Comment,
    Partial,
    Section
}

pub struct Parser {
    input: String
}

impl Parser {
    // Capture all regex matches for rustache tags and return them as a vector of
    // string slices after parsing their tag types. Results will be used by the 
    //parser in order to create the TagMap.
    fn find_token_matches(input: &str) -> Vec<(uint, &str, &str)>{
        let mut result = Vec::new();
        let re = regex!(r"(\{\{*\S?\s?[\w\s]*\s?\S?\}\})");
        for cap in re.captures_iter(input) {
            let (start, end) = cap.pos(1).unwrap();
            let mut name = cap.at(1);
            let mut tag_type  = match name.char_at(2) {
                '&' => Unescaped,
                '{' => Unescaped,
                '!' => Comment,
                '>' => Partial,
                '#' => Section,
                _   => Partial
            };

            let mut token = (start, name, tag_type);
            result.push(token);
        }

        result
    }


}

