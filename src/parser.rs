//! A simple parser for parsing rustache files.
//!
//! Can parse parse opening and closing rustaches and text nodes.

use std::collections::hashmap::HashSet;
use std::io::{File, BufferedReader};

#[deriving(Show)]
pub enum Node {
    Static(String),
    Value(String)
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
        for (i, c) in line.chars().enumerate() {
            if c == '{' && line.char_at(i+1) == '{' {
                open_pos = i;
                nodes.push(Static(line.slice(close_pos, open_pos).to_string()));
            }
            if c == '}' && i < len - 1 && line.char_at(i+1) == '}' {
                close_pos = i+2;
                nodes.push(Value(line.slice(open_pos, close_pos).to_string()));
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
            };        
        }

        tag_map
    }
}

#[test]
fn test_tokenize() {
    let test: &str = "Not a tag {{ tag }}.  Yep {{ tag }}";
    Parser::tokenize_line(test);
    assert!(false);
}

#[test]
fn test_token_mapper() {
    let test: &str = "Not a tag {{ tag1 }}.  Yep {{ tag2 }}";
    let nodes = Parser::tokenize_line(test);
    let set = Parser::create_token_map_from_tags(nodes);
    println!("{}", set);
    assert!(false);
}