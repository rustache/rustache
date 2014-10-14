// The parser processes a list of mustache tokens created in
// the compiler into a list of templater useable nodes.
// Nodes contain only the necessary information to be used
// to seek out appropriate data for injection.

use compiler::{Token, Text, Variable, OTag, CTag, Raw, Partial, Comment};

// Node signifies the data structure used by the template to
// determine how to correctly implement data.  Each Node type
// stores the variable name as well as the raw tag for use by
// lambdas.

#[deriving(PartialEq, Eq, Clone, Show)]
pub enum Node<'a> {
    Static(&'a str), // (text)
    Value(&'a str, String), // (name, tag)
    Section(&'a str, Vec<Node<'a>>, bool, String, String), // (name, children, inverted, otag, ctag)
    Unescaped(&'a str, String), // (name, tag)
    Part(&'a str, &'a str) // // (name, tag)
}

#[deriving(PartialEq, Eq)]
enum ParserStatus {
    Parse,
    Skip
}

// Function that recursively handles tag names that utilize dot notation
// shorthand.
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
            return Section(variable, vec![handle_dot_notation(parts.slice_from(1), unescaped, amp)], false, otag, ctag);
        }
    }
}

// Parse_nodes signifies the parser entry point passed the results received from
// the template compiler.
pub fn parse_nodes<'a>(list: &Vec<Token<'a>>) -> Vec<Node<'a>> {
    let mut nodes: Vec<Node> = vec![];
    let mut it = list.iter().enumerate().peekable();
    let mut status = Parse;

    loop {
        match it.next() {
            Some((i, &token)) => {
                match token {
                    Comment => {
                        // Check the next element for whitespace
                        match it.peek() {
                            Some(&(_, token)) => {
                                match token {
                                    &Text(ref value) => {
                                        // if whitespace and should skip, advance to next token
                                        if value.is_whitespace() && status == Skip {
                                            status = Parse;
                                            it.next();
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            None => continue
                        }
                    },
                    Text(text) => {
                        match text.contains_char('\n') {
                            true => {
                                status = Skip;
                            },
                            false => {}
                        }
                        nodes.push(Static(text))
                    },
                    Variable(name, raw) => {
                        let dot_notation = name.contains_char('.');
                        match dot_notation {
                            false => nodes.push(Value(name, raw.to_string())),
                            true => {
                                let parts: Vec<&str> = name.split_str(".").collect();
                                let node = handle_dot_notation(parts.as_slice(), false, false);
                                nodes.push(node);
                            }
                        }
                    },
                    Raw(name, raw) => {
                        let dot_notation = name.contains_char('.');
                        let ampersand = raw.contains_char('&');
                        match dot_notation {
                            false => nodes.push(Unescaped(name, raw.to_string())),
                            true => {
                                let parts: Vec<&str> = name.split_str(".").collect();
                                match ampersand {
                                    true => {
                                        let node = handle_dot_notation(parts.as_slice(), true, true);
                                        nodes.push(node);
                                    },
                                    false => {
                                        let node = handle_dot_notation(parts.as_slice(), true, false);
                                        nodes.push(node);
                                    }
                                };
                            }
                        }
                    }
                    Partial(name, raw) => nodes.push(Part(name, raw)),
                    CTag(_, _) => {
                        // CTags that are processed outside of the context of a 
                        // corresponding OTag are ignored.
                        continue;
                    },
                    OTag(name, inverted, raw) => {
                        let mut children: Vec<Token> = vec![];
                        let mut count = 0u;
                        let mut otag_count = 1u;
                        for item in list.slice_from(i + 1).iter() {
                            count += 1;
                            match *item {
                                OTag(title, inverted, raw) => {
                                    if title == name {
                                        otag_count += 1;
                                    }
                                    children.push(*item);
                                },
                                CTag(title, temp) => {
                                    if title == name && otag_count == 1 {
                                        nodes.push(Section(name, parse_nodes(&children).clone(), inverted, raw.to_string(), temp.to_string()));
                                        break;
                                    } else if title == name && otag_count > 1 {
                                        otag_count -= 1;
                                        children.push(*item);
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
                        // OTag is never closed, these children will never be processed.
                        while count > 1 {
                            it.next();
                            count -= 1;
                        }
                    }
                }
            },
            None => break
        }

    }

    // Return the populated list of nodes for use by the template engine
    nodes
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
