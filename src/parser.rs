// The parser processes a list of mustache tokens created in
// the compiler into a list of templater useable nodes.
// Nodes contain only the necessary information to be used
// to seek out appropriate data for injection.

use compiler::{Token, Text, Variable, OTag, CTag, Raw, Partial};
use std::mem;


#[deriving(PartialEq, Eq, Clone, Show)]
pub enum Node<'a> {
    Static(&'a str),
    Value(&'a str),
    // (name, children, inverted)
    Section(&'a str, Vec<Node<'a>>, bool),
    Unescaped(&'a str),
    File(&'a str)
}

// The parser is instantiated with a list of tokens, and immediately
// goes to work to create a node list from that list.
pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    pub nodes: Vec<Node<'a>>
}

impl<'a> Parser<'a> {
    pub fn new<'a>(tokens: &'a Vec<Token<'a>>) -> Parser<'a> {
        let mut parser = Parser {
            tokens: tokens,
            nodes: vec![]
        };
        let mut nodes = parser.parse_nodes(parser.tokens);
        mem::swap(&mut parser.nodes, &mut nodes);
        parser
    }

    fn parse_nodes<'a>(&self, list: &Vec<Token<'a>>) -> Vec<Node<'a>> {
        let mut nodes: Vec<Node> = vec![];
        let mut it = list.iter().enumerate();

        loop {
            match it.next() {
                Some((i, &token)) => {
                    match token {
                        Text(text) => nodes.push(Static(text)),
                        Variable(name) => nodes.push(Value(name)),
                        Raw(name) => nodes.push(Unescaped(name)),
                        Partial(name) => nodes.push(File(name)),
                        CTag(_) => {
                            // CTags that are processed outside of the context of a 
                            // corresponding OTag are ignored.
                            continue;
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
                                    _ => {
                                        children.push(*item);
                                        continue;
                                    }
                                }
                            }

                            // Advance the iterator to the position of the CTAG.  If the 
                            //OTag is never closed, these children will never be processed.
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

#[cfg(test)]
mod parser_tests {
    use compiler::{Compiler, Token, Text, Variable, OTag, CTag, Raw, Partial};
    use parser::{Parser, Node, Static, Value, Section, Unescaped, File};
    use std::mem;

    #[test]
    fn parse_static() {
        let contents = "Static String {{ token }}{{# section }}{{ child_tag }}{{/ section }}{{> new }}";
        let compiler = Compiler::new(contents);
        let parser = Parser::new(&compiler.tokens);
        let static_node = Static("Static String ");
        let expected: Vec<Node> = vec![static_node];
    }

    #[test]
    fn parse_value() {
        let contents = "{{ token }}";
        let compiler = Compiler::new(contents);
        let parser = Parser::new(&compiler.tokens);
        let value_node = Value("token");
        let expected: Vec<Node> = vec![value_node];
    }

    #[test]
    fn parse_section() {
        let contents = "{{# section }}{{ child_tag }}{{/ section }}";
        let compiler = Compiler::new(contents);
        let parser = Parser::new(&compiler.tokens);
        let section_node = Section("section", vec![Value("child_tag")], false);
        let expected: Vec<Node> = vec![section_node];
    }

    #[test]
    fn parse_inverted() {
        let contents = "{{^ inverted }}{{ child_tag }}{{/ inverted }}";
        let compiler = Compiler::new(contents);
        let parser = Parser::new(&compiler.tokens);
        let inverted_node = Section("inverted", vec![Value("child_tag")], true);
        let expected: Vec<Node> = vec![inverted_node];
    }

    #[test]
    fn parse_unescaped() {
        let contents = "{{& unescaped }}";
        let compiler = Compiler::new(contents);
        let parser = Parser::new(&compiler.tokens);
        let undescaped_node = Unescaped("unescaped");
        let expected: Vec<Node> = vec![undescaped_node];
    }

    #[test]
    fn parse_partial() {
        let contents = "Static String {{ token }}{{# section }}{{ child_tag }}{{/ section }}{{> new }}";
        let compiler = Compiler::new(contents);
        let parser = Parser::new(&compiler.tokens);
        let file_node = File("new");
        let expected: Vec<Node> = vec![file_node];
    }

    #[test]
    fn parse_all() {
        let contents = "Static String {{ token }}{{# section }}{{ child_tag }}{{/ section }}{{> new }}{{& unescaped }}";
        let compiler = Compiler::new(contents);
        let parser = Parser::new(&compiler.tokens);
        let static_node = Static("Static String ");
        let value_node = Value("token");
        let section_node = Section("section", vec![Value("child_tag")], false);
        let file_node = File("new");
        let undescaped_node = Unescaped("unescaped");
        let expected: Vec<Node> = vec![static_node, value_node, section_node, file_node, undescaped_node];
        assert_eq!(expected, parser.nodes);
    }
}
