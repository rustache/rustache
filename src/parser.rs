//! A simple parser for parsing rustache files.
//!
//! Can parse parse opening and closing rustaches and text nodes.

use std::collections::hashmap::HashSet;
use std::io::{File, BufferedReader};

#[deriving(Show)]
pub enum Node {
    Static(String),
    Value(String),
    OTag(Option<String>),
    CTag(Option<String>),
    Inverted(String),
    Unescaped(String),
}

#[deriving(Show)]
pub struct Parser<'a>;

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser
    }

    pub fn read_template(template_path: &str) -> Vec<String> {
        let path = Path::new(template_path);
        let mut file = BufferedReader::new(File::open(&path));
        let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

        lines
    }

    pub fn tokenize_line(line: &str) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
        let mut open_pos = 0u;
        let mut close_pos = 0u;
        let len = line.len();
        for (mut i, c) in line.chars().enumerate() {
            if c == '{' && line.char_at(i+1) == '{' {
                open_pos = i;
                if open_pos != close_pos {
                nodes.push(Static(line.slice(close_pos, open_pos).to_string()));
                }
                i += 1;
            }
            if c == '}' && i < len - 1 && line.char_at(i+1) == '}' {
                close_pos = i + 2;
                let val = line.slice(open_pos + 2u, close_pos - 2u);
                match val.char_at(0) {
                    '!' => continue, // comment, skip over
                    '#' => nodes.push(OTag(Some(val.slice_from(1).trim().to_string()))), // OTAG
                    '/' => nodes.push(CTag(Some(val.slice_from(1).trim().to_string()))), // CTAG
                    '^' => nodes.push(Inverted(val.slice_from(1).trim().to_string())), // inverted
                    '>' => continue, // partial
                    '&' => nodes.push(Unescaped(val.slice_from(1).trim().to_string())), // unescaped literal
                    '{' => continue, // unescaped literal
                    _ => nodes.push(Value(val.trim().to_string()))

                }
                i += 2;
            }
            
        }
        if (close_pos < len) {
            nodes.push(Static(line.slice_from(close_pos).to_string()));
        }

        nodes
    }

    pub fn create_token_map_from_tags<'a>(nodes: Vec<Node>) -> HashSet<String> {
        let mut tag_map: HashSet<String> = HashSet::new();
        for node in nodes.iter() {
            match *node {
                Value(ref text)  => tag_map.insert(text.clone()),
                Static(ref text) => continue,
                OTag(ref opt) => continue,
                CTag(ref opt) => continue,
                Inverted(ref text)  => continue,
                Unescaped(ref text)  => continue,
            };        
        }

        tag_map
    }
}

// #[test]
// fn test_tokenize() {
//     let test: &str = "Not a tag {{ tag }}.  Yep {{ tag }}";
//     Parser::tokenize_line(test);
//     assert!(false);
// }

#[test]
fn test_token_mapper() {
    let test: &str = "Static tag!{{normal}}{{! comment }}!{{# tag }} {{/ tag }} {{^      inverted }} {{& unescaped }}";
    let nodes = Parser::tokenize_line(test);
    for node in nodes.iter() {
        println!("{}", node);
    }
    let set = Parser::create_token_map_from_tags(nodes);
    println!("{}", set);
    assert!(false);
}