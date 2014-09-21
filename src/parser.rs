//! A simple parser for parsing rustache files.
//!
//! Can parse parse opening and closing rustaches and text nodes.

use std::collections::hashmap::HashSet;
use std::io::{File};

pub struct Node<'a> {
    pub val: String,
    pub node_type: Tag<'a>
}

impl<'a> Node<'a> {
    pub fn new(val: String, tag: Tag) -> Node<'a> {
        Node {
            val: val,
            node_type: tag
        }
    }
}
#[deriving(Show, PartialEq, Eq)]
pub enum Tag<'a> {
    Text,
    Value,
}

#[deriving(Show)]
pub struct Parser<'a>;

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser
    }

    pub fn read_template(template_path: &str) -> String {
        let path = Path::new(template_path);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => fail!("Couldn't open {}: {}", display, why.desc),
            Ok(file) => file,
        };

        // read file to string 
        let template_str: String = match file.read_to_string() {
            Err(why)   => fail!("Couldn't read {}: {}", display, why.desc),
            Ok(string) =>  string,
        };

        template_str
    }

    pub fn tag_lines<'a>(file: String) -> Vec<Node<'a>> {
        let mut nodes: Vec<Node> = Vec::new();
        let line_regex = regex!("\n");
        let mustache_regex = regex!(r"\{\{(\s?[\w\s]*\s?)\}\}");
        let lines: Vec<&str> = line_regex.split(file.as_slice()).collect();
        for line in lines.iter() {
            let val = mustache_regex.is_match(*line);
            let node = match val {
                true  => Node::new(line.trim().to_string(), Value),
                false => Node::new(line.trim().to_string(), Text)
            };
            nodes.push(node);
        }

        nodes
    }

    pub fn create_token_map_from_tags<'a>(nodes: &'a Vec<Node>) -> HashSet<&'a str> {
        let mut tag_map: HashSet<&str> = HashSet::new();
        for node in nodes.iter() {
            match node.node_type {
                Text => continue,
                Value  => tag_map.insert(node.val.as_slice())
            };        
        }

        tag_map
    }
}