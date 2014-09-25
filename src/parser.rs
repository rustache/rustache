// use std::collections::hashmap::HashSet;
use compiler::{Token, Text, Variable, OTag, CTag, Raw, Partial};
use std::mem;

#[deriving(Show, PartialEq, Eq, Clone)]
pub enum Node {
    Static(&'static str),
    Value(&'static str),
    Section(&'static str, Vec<Node>, bool), // (name, children, inverted)
    Unescaped(&'static str),
    File(&'static str)
}

#[deriving(Show)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pub nodes: Vec<Node>
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

        let mut it = list.iter().enumerate();
        loop {
            match it.next() {
                Some((i, &token)) => {
                    match token {
                        Text(text)           => nodes.push(Static(text)),
                        Variable(name)       => nodes.push(Value(name)),
                        Raw(name)            => nodes.push(Unescaped(name)),
                        Partial(name)        => nodes.push(File(name)),
                        CTag(name)           => {
                            continue;
                            // fail!("Incorrect syntax in template, {} closed without being opened", name);
                        },
                        OTag(name, inverted) => {
                            let mut children: Vec<Token> = vec![];
                            let mut count = 0u;
                            for item in list.slice_from(i + 1).iter() {
                                count += 1;
                                match *item {
                                    CTag(title) => {
                                        if title == name {
                                            nodes.push(Section(name, self.parse_nodes(&children).clone(), inverted));
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
                            }
                            while count > 1 {
                                it.next();
                                count -= 1;
                            }
                        },
                    }
                },
                None => break
            }
        }
        
        nodes
    }
}

#[test]
fn test_parser() {
    use compiler::Compiler;

    let contents = "Static String {{ token }}{{# section }}{{ child_tag }}{{/ section }}{{> new }}";
    let compiler = Compiler::new(contents);
    let parser = Parser::new(&compiler.tokens);
    let static_node  = Static("Static String ");
    let value_node   = Value("token");
    let section_node = Section("section", vec![Value("child_tag")], false);
    let file_node    = File("new");
    let expected: Vec<Node> = vec![static_node, value_node, section_node, file_node];
    assert_eq!(expected, parser.nodes);
}
