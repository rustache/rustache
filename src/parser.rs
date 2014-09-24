// //! A simple parser for parsing rustache files.
// //!
// //! Can parse parse opening and closing rustaches and text nodes.

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
        let len = list.len();
        for i in range(0, len) {
            match list[i] {
                Text(text)           => nodes.push(Static(text)),
                Variable(name)       => nodes.push(Value(name)),
                Raw(name)            => nodes.push(Unescaped(name)),
                OTag(name, inverted) => {
                    let mut children: Vec<Token> = vec![];
                    // iterate through the rest of the list (adding to children)
                    // until the closing tag is found
                    // for elem in list.iter().fold().flat_map().collect() {
                    for j in range(i, len) {
                        let elem = list[j];
                        match elem {
                            CTag(title) => {
                                if title == name {
                                    // advance the iterator past the child nodes
                                    nodes.push(Section(name, self.parse_nodes(&children), inverted));
                                    break;
                                } else {
                                    children.push(elem);
                                    continue;
                                }
                            },
                            _           => {
                                // if j == len - 1 {
                                    // fail!("Incorrect syntax in template");
                                // }
                                children.push(elem);
                                continue;
                            }
                        }
                    }
                },
                CTag(name)           => {
                    // If this is triggered, its a closing tag that is present without
                    // an opening tag.  Representing bad syntax, error or ignore?
                    fail!("Incorrect syntax in template, {} closed without being opened", name);
                },
            }
        }

        // I cant completely understand this...
        // list.iter().enumerate().map(|(pos, elem)| (pos == list.len() - 1, elem)).scan(None, |&mut ctag_state, (is_final, elem)| {
        //     match *ctag_state {
        //         Some(ref closing_name, ref mut children, ref inverted) => match elem {
        //             CTag(name) if name == *closing_name {
        //                 Some(Section(name, self.parse_nodes(children), inverted))
        //             },
        //             child if !is_final => { children.push(child); None },
        //         },
        //         None => match elem {
        //             Text(text) => Some(Static(text)),
        //             /* others omitted */
        //             OTag(name, inverted) => {
        //                 *ctag_state = (name, vec![], inverted);
        //                 None
        //             },
        //             CTag(_) => None
        //         }
        //     }
        // }).collect();

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
// fn tokenize_should_map_strings() {
//     let test: &str = "Static tag!{{normal}}{{! comment }}!{{# tag }} {{/ tag }} {{^ inverted }} {{& unescaped }}";
//     let nodes = Parser::tokenize(test);
//     //should contain static blocks
//     assert_eq!(nodes.contains(&Static("Static tag!".to_string())), true);
//     //should not contain comment blocks
//     assert_eq!(nodes.contains(&Value("comment".to_string())), false);
//     //should contain open and close blocks
//     assert_eq!(nodes.contains(&OTag(Some("tag".to_string()))), true);
//     //should not contain unescaped blocks
//     assert_eq!(nodes.contains(&Unescaped("unescaped".to_string())), true);
// }

// #[test]
// fn mapper_should_create_a_set_of_useable_variables() {
//     let nodes = vec![Static("Static tag!".to_string()), Value("comment".to_string()), OTag(Some("tag".to_string()))];
//     let set = Parser::create_map_from_tokens(nodes);

//     // should only contain value nodes
//     assert!(set.contains(&"comment".to_string()));
// }
