// use std::collections::hashmap::HashSet;
use compiler::{Token, Text, Variable, OTag, CTag, Raw};
use std::mem;

#[deriving(Show, PartialEq, Eq, Clone)]
pub enum Node {
    Static(&'static str),
    Value(&'static str),
    Section(&'static str, Vec<Node>, bool), // (name, children, inverted)
    Unescaped(&'static str),
}

#[deriving(Show)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    nodes: Vec<Node>
}

impl<'a> Parser<'a> {
    pub fn new<'a>(tokens: &'a Vec<Token>) -> Parser<'a> {
        let mut parser = Parser {
            tokens: tokens,
            nodes: vec![]
        };
        let mut nodes = parser.parse_nodes(parser.tokens);
        mem::swap(&mut parser.nodes, &mut nodes);
        parser
    }

    fn parse_nodes(&self, list: &'a Vec<Token>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
        let mut il = list.iter().enumerate();
        for (i, token) in il {
            match *token {
                Text(text)           => nodes.push(Static(text)),
                Variable(name)       => nodes.push(Value(name)),
                Raw(name)            => nodes.push(Unescaped(name)),
                OTag(name, inverted) => {
                    let mut children: Vec<Token> = vec![];
                    for (j, item) in list.slice_from(i).iter().enumerate() {
                        match *item {
                            CTag(title) => {
                                if title == name {
                                    nodes.push(Section(name, self.parse_nodes(&children), inverted));
                                    break;
                                } else {
                                    children.push(*item);
                                    continue;
                                }
                            },
                            _           => {
                                children.push(*item);
                                continue;
                            }
                        }
                        il.next();
                    }
                },
                CTag(name)           => {
                    continue;
                    fail!("Incorrect syntax in template, {} closed without being opened", name);
                },
            }
        }

        nodes
    }
}

//     pub fn create_map_from_tokens<'a>(nodes: Vec<Node>) -> HashSet<String> {
//         let mut tag_map: HashSet<String> = HashSet::new();
//         for node in nodes.iter() {
//             match *node {
//                 Value(ref text)  => tag_map.insert(text.clone()),
//                 Static(ref text) => continue,
//                 OTag(ref opt) => continue,
//                 CTag(ref opt) => continue,
//                 Inverted(ref text)  => continue,
//                 Unescaped(ref text)  => continue,
//             };        
//         }

//         tag_map
//     }
// }

#[test]
fn test_parser() {
    use compiler::Compiler;

    let contents = "Static String {{ token }} {{# section }}{{ child_tag }}{{/ section }}";
    let compiler = Compiler::new(contents);
    let parser = Parser::new(&compiler.tokens);
    for node in parser.nodes.iter() {
        println!("{}", node);
    }
    assert!(false);
}

// #[test]
// fn mapper_should_create_a_set_of_useable_variables() {
//     let nodes = vec![Static("Static tag!".to_string()), Value("comment".to_string()), OTag(Some("tag".to_string()))];
//     let set = Parser::create_map_from_tokens(nodes);

//     // should only contain value nodes
//     assert!(set.contains(&"comment".to_string()));
// }
