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

// pub struct Node<'a> {
//     pub val: String,
//     pub node_type: Tag<'a>
// }

// impl<'a> Node<'a> {
//     pub fn new(val: String, tag: Tag) -> Node<'a> {
//         Node {
//             val: val,
//             node_type: tag
//         }
//     }
// }
// #[deriving(Show, PartialEq, Eq)]
// pub enum Tag<'a> {
//     Text,
//     Value,
// }

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

    // pub fn tag_lines<'a>(lines: Vec<String>) -> Vec<Node<'a>> {
    //     let mut nodes: Vec<Node> = Vec::new();
    //     let re = regex!(r"\{\{\S?(\s?[\w\s]*\s?\S?)\}\}");
    //     for line in lines.iter() {
    //         if re.is_match(line.as_slice()) {
    //             for cap in re.captures_iter(line.as_slice()) {
    //                 let (s, e) = cap.pos(0).unwrap();
    //                 let start = Node::new(line.as_slice().slice_to(s).trim().to_string(), Text);
    //                 nodes.push(start);
    //                 let tag = Node::new(cap.at(1).trim().to_string(), Value);
    //                 nodes.push(tag);
    //                 let end = Node::new(line.as_slice().slice_from(e).trim().to_string(), Text);
    //                 nodes.push(end);
    //             }
    //         } else {
    //             let node = Node::new(line.as_slice().trim().to_string(), Text);
    //             nodes.push(node);
    //         }
    //     }
    //     nodes
    // }

    pub fn tokenize_line(line: &str) {
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
        for node in nodes.iter() {
            println!("{}", node);
        }

    }

    // pub fn create_token_map_from_tags<'a>(nodes: &'a Vec<Node>) -> HashSet<&'a str> {
    //     let mut tag_map: HashSet<&str> = HashSet::new();
    //     for node in nodes.iter() {
    //         match node.node_type {
    //             Text => continue,
    //             Value  => tag_map.insert(node.val.as_slice())
    //         };        
    //     }

    //     tag_map
    // }
}

#[test]
fn test_tokenize() {
    let test: &str = "Not{{!tag }}Yep{{#tag2}}";
    Parser::tokenize_line(test);
    assert!(false);
}