// The parser processes a list of mustache tokens created in
// the compiler into a list of templater useable nodes.
// Nodes contain only the necessary information to be used
// to seek out appropriate data for injection.

use compiler::Token;
use compiler::Token::{Text, Variable, OTag, CTag, Raw, Partial, Comment};
use self::Node::*;
use self::ParserStatus::*;

// Node signifies the data structure used by the template to
// determine how to correctly implement data.  Each Node type
// stores the variable name as well as the raw tag for use by
// lambdas.

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Node<'a> {
    Static(&'a str), // (text)
    Value(&'a str, String), // (name, tag)
    Section(&'a str, Vec<Node<'a>>, bool, String, String), // (name, children, inverted, otag, ctag)
    Unescaped(&'a str, String), // (name, tag)
    Part(&'a str, &'a str) // // (name, tag)
}

#[derive(PartialEq, Eq, Debug)]
enum ParserStatus {
    Parse,
    // Sect,
    Skip
}

trait LocalStringExt {
    fn is_whitespace(&self) -> bool;
}

impl LocalStringExt for String {
    fn is_whitespace(&self) -> bool {
        self.chars().all(|c| c.is_whitespace())
    }
}

impl LocalStringExt for str {
    fn is_whitespace(&self) -> bool {
        self.chars().all(|c| c.is_whitespace())
    }
}

// Parse list of tokens into instruction nodes
// Section nodes will be handled recursively
pub fn parse_nodes<'a>(list: &Vec<Token<'a>>) -> Vec<Node<'a>> {
    let mut nodes: Vec<Node> = vec![];
    let mut it = list.iter().enumerate().peekable();
    let mut status = Parse;

    loop {
        // Iterate while still nodes in the list
        match it.next() {
            Some((i, token)) => {
                match token {
                    &Text(text) => nodes.push(parse_text_node(text, &mut status)),
                    &Variable(name, raw) => nodes.push(parse_variable_node(name, raw)),
                    &Raw(name, raw) => nodes.push(parse_raw_node(name, raw)),
                    &Partial(name, raw) => nodes.push(Part(name, raw)),
                    // Unopened closing tags are ignored
                    // TODO: Return a parser error?
                    &CTag(_, _) => continue,
                    &OTag(name, inverted, raw) => {
                        let mut children: Vec<Token<'a>> = vec![];
                        let mut count = 0u32;
                        let mut otag_count = 1u32;
                        for item in list[i + 1 ..].iter() {
                            count += 1;
                            match *item {
                                OTag(title, _, _) => {
                                    if title == name {
                                        otag_count += 1;
                                    }
                                    children.push((*item).clone());
                                },
                                CTag(title, temp) => {
                                    if title == name && otag_count == 1 {
                                        nodes.push(Section(name, parse_nodes(&children).clone(), inverted, raw.to_string(), temp.to_string()));
                                        break;
                                    } else if title == name && otag_count > 1 {
                                        otag_count -= 1;
                                        children.push((*item).clone());
                                    } else {
                                        children.push((*item).clone());
                                        continue;
                                    }
                                },
                                _ => {
                                    children.push((*item).clone());
                                    continue;
                                }
                            }
                        }

                        // Advance the iterator to the position of the CTAG.  If the
                        // OTag is never closed, these children will never be processed.
                        // TODO: Return a parser warning in the case of an unclosed tag?
                        while count > 1 {
                            it.next();
                            count -= 1;
                        }
                    },
                    &Comment => {
                        // Check the next element for whitespace
                        match it.peek() {
                            Some(&(_, token)) => {
                                match parse_comment_node(token, &mut status, &mut nodes) {
                                    true => {
                                        // it.next();
                                    },
                                    false => {}
                                }
                            },
                            None => {
                                match nodes.last().unwrap() {
                                    &Static(text) => {
                                        if text.is_whitespace() {
                                            nodes.pop();
                                        }
                                    }
                                    _ => continue,
                                }
                            },
                        }
                    },
                }
            },
            None => break
        }
    }

    // Return the populated list of nodes
    nodes
}

// Helper function for handling the creation of a text node
fn parse_text_node<'a>(text: &'a str, status: &mut ParserStatus) -> Node<'a> {
    match *status {
        _ => {
            if text.contains("\n") {
                *status = Skip;
            } else if text.is_whitespace() {
                *status = Skip;
            }
            return Static(text);
        }
    }
}

// Helper function for handling the creation of a variable node
fn parse_variable_node<'a>(name: &'a str, raw: &'a str) -> Node<'a> {
    let dot_notation = name.contains(".");
    match dot_notation {
        false => return Value(name, raw.to_string()),
        true => {
            let parts: Vec<&str> = name.split(".").collect();
            let node = handle_dot_notation(parts.as_slice(), false, false);
            return node;
        }
    }
}

// Helper function for handling the creation of an unescaped variable node
fn parse_raw_node<'a>(name: &'a str, raw: &'a str) -> Node<'a> {
    let dot_notation = name.contains(".");
    let ampersand = raw.contains("&");
    match dot_notation {
        false => {
            return Unescaped(name, raw.to_string());
        }
        true => {
            let parts: Vec<&str> = name.split(".").collect();
            match ampersand {
                true => {
                    let node = handle_dot_notation(parts.as_slice(), true, true);
                    return node;
                },
                false => {
                    let node = handle_dot_notation(parts.as_slice(), true, false);
                    return node;
                }
            };
        }
    }
}

// Helper function for handling the creation of comment nodes and
// properly handle whitespace
fn parse_comment_node<'a>(token: &Token, status: &mut ParserStatus, nodes: &mut Vec<Node<'a>>) -> bool {
    match *token {
        Text(ref value) => {
            match *status {
                Skip => {
                    // If whitespace and should skip, advance to next token
                    if value.is_whitespace() {
                        match nodes.last().unwrap() {
                            &Static(text) => {
                                // If the previous node is whitespace and has a newline
                                // then remove it
                                if text.is_whitespace() && text.contains("\n") {
                                    nodes.pop();
                                }
                            },
                            _ => {}
                        }
                        *status = Parse;
                        return true;
                    } else {
                        *status = Parse;
                        return false;
                    }
                },
                Parse => return false,
                // Sect => return false,
            }
        },
        _ => return false
    }
}

// Recursively handle tag names that utilize dot notation shorthand
fn handle_dot_notation<'a>(parts: &[&'a str], unescaped: bool, amp: bool) -> Node<'a> {
    let variable = parts[0];
    match parts.len() {
        // Determine if the remaining portion of the tag name is the
        // variable or another section.
        1 => {
            match unescaped {
                // Determine if the matching tag is unesecaped or normal.
                true => {
                    match amp {
                        true => {
                            let mut var = "{{&".to_string();
                            var.push_str(variable);
                            var.push_str("}}");
                            return Unescaped(variable, var);
                        },
                        false => {
                            let mut var = "{{{".to_string();
                            var.push_str(variable);
                            var.push_str("}}}");
                            return Unescaped(variable, var);
                        }
                    }
                }
                false => {
                    let mut var = "{{".to_string();
                    var.push_str(variable);
                    var.push_str("}}");
                    return Value(variable, var);
                }
            }
        }
        _ => {
            let mut otag = "{{#".to_string();
            let mut ctag = "{{/".to_string();

            otag.push_str(variable);
            otag.push_str("}}");
            ctag.push_str(variable);
            ctag.push_str("}}");

            // Enter recursion and assign the results as children.
            return Section(variable, vec![handle_dot_notation(&parts[1..], unescaped, amp)], false, otag, ctag);
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use compiler::{Token, Text, Variable, OTag, CTag, Raw, Partial};
    use parser;
    use parser::{Node, Static, Value, Section, Unescaped, Part};

    #[test]
    fn parse_dot_notation_simple() {
        let tokens: Vec<Token> = vec![Variable("section.child_tag", "{{ section.child_tag }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Section("section", vec![Value("child_tag", "{{child_tag}}".to_string())], false, "{{#section}}".to_string(), "{{/section}}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_dot_notation_triple_mustache() {
        let tokens: Vec<Token> = vec![Raw("section.child_tag", "{{{ section.child_tag }}}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Section("section", vec![Unescaped("child_tag", "{{{child_tag}}}".to_string())], false, "{{#section}}".to_string(), "{{/section}}".to_string())];
        assert_eq!(nodes, expected);
    }

        #[test]
    fn parse_dot_notation_ampersand() {
        let tokens: Vec<Token> = vec![Raw("section.child_tag", "{{& section.child_tag }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Section("section", vec![Unescaped("child_tag", "{{&child_tag}}".to_string())], false, "{{#section}}".to_string(), "{{/section}}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_nested_dot_notation_basic() {
        let tokens: Vec<Token> = vec![Variable("section.child.tag", "{{ section.child.tag }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![
            Section("section", vec![
                Section("child", vec![
                    Value("tag", "{{tag}}".to_string())]
                    ,false, "{{#child}}".to_string(), "{{/child}}".to_string())]
            , false, "{{#section}}".to_string(), "{{/section}}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_nested_dot_notation_triple_mustache() {
        let tokens: Vec<Token> = vec![Raw("section.child.tag", "{{{ section.child.tag }}}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![
            Section("section", vec![
                Section("child", vec![
                    Unescaped("tag", "{{{tag}}}".to_string())]
                    ,false, "{{#child}}".to_string(), "{{/child}}".to_string())]
            , false, "{{#section}}".to_string(), "{{/section}}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_nested_dot_notation_ampersand() {
        let tokens: Vec<Token> = vec![Raw("section.child.tag", "{{& section.child.tag }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![
            Section("section", vec![
                Section("child", vec![
                    Unescaped("tag", "{{&tag}}".to_string())]
                    ,false, "{{#child}}".to_string(), "{{/child}}".to_string())]
            , false, "{{#section}}".to_string(), "{{/section}}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_static() {
        let tokens: Vec<Token> = vec![Text("Static String ")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Static("Static String ")];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_value() {
        let tokens: Vec<Token> = vec![Variable("token", "{{ token }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Value("token", "{{ token }}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_section() {
        let tokens: Vec<Token> = vec![OTag("section", false, "{{# section }}"), Variable("child_tag", "{{ child_tag }}"), CTag("section", "{{/ section }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Section("section", vec![Value("child_tag", "{{ child_tag }}".to_string())], false, "{{# section }}".to_string(), "{{/ section }}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_inverted() {
        let tokens: Vec<Token> = vec![OTag("inverted", true, "{{^ inverted }}"), Variable("child_tag", "{{ child_tag }}"), CTag("inverted", "{{/ inverted }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Section("inverted", vec![Value("child_tag", "{{ child_tag }}".to_string())], true, "{{^ inverted }}".to_string(), "{{/ inverted }}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_unescaped() {
        let tokens: Vec<Token> = vec![Raw("unescaped", "{{& unescaped }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Unescaped("unescaped", "{{& unescaped }}".to_string())];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_partial() {
        let tokens: Vec<Token> = vec![Partial("new","{{> new }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Part("new", "{{> new }}")];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_all() {
        let tokens: Vec<Token> = vec![
            Text("Static String "), Variable("token", "{{ token }}"), OTag("section", false, "{{# section }}"),
            Variable("child_tag", "{{ child_tag }}"), CTag("section", "{{/ section }}"),
            Partial("new","{{> new }}"), Raw("unescaped", "{{& unescaped }}")
        ];
        let nodes = parser::parse_nodes(&tokens);
        let static_node = Static("Static String ");
        let value_node = Value("token", "{{ token }}".to_string());
        let section_node = Section("section", vec![Value("child_tag", "{{ child_tag }}".to_string())], false, "{{# section }}".to_string(), "{{/ section }}".to_string());
        let file_node = Part("new", "{{> new }}");
        let undescaped_node = Unescaped("unescaped", "{{& unescaped }}".to_string());
        let expected: Vec<Node> = vec![static_node, value_node, section_node, file_node, undescaped_node];
        assert_eq!(nodes, expected);
    }
}
