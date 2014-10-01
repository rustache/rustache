// The parser processes a list of mustache tokens created in
// the compiler into a list of templater useable nodes.
// Nodes contain only the necessary information to be used
// to seek out appropriate data for injection.

use compiler::{Token, Text, Variable, OTag, CTag, Raw, Partial};

#[deriving(PartialEq, Eq, Clone, Show)]
pub enum Node<'a> {
    Static(&'a str),
    Value(&'a str, &'a str),
    // (name, children, inverted)
    Section(&'a str, Vec<Node<'a>>, bool, &'a str, &'a str),
    Unescaped(&'a str, &'a str),
    Part(&'a str, &'a str)
}
pub fn parse_nodes<'a>(list: &Vec<Token<'a>>) -> Vec<Node<'a>> {
    let mut nodes: Vec<Node> = vec![];
    let mut it = list.iter().enumerate();

    loop {
        match it.next() {
            Some((i, &token)) => {
                match token {
                    Text(text) => nodes.push(Static(text)),
                    Variable(name, raw) => nodes.push(Value(name, raw)),
                    Raw(name, raw) => nodes.push(Unescaped(name, raw)),
                    Partial(name, raw) => nodes.push(Part(name, raw)),
                    CTag(_, _) => {
                        // CTags that are processed outside of the context of a 
                        // corresponding OTag are ignored.
                        continue;
                    },
                    OTag(name, inverted, raw) => {
                        let mut children: Vec<Token> = vec![];
                        let mut count = 0u;
                        for item in list.slice_from(i + 1).iter() {
                            count += 1;
                            match *item {
                                CTag(title, temp) => {
                                    if title == name {
                                        nodes.push(Section(name, parse_nodes(&children).clone(), inverted, raw, temp));
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


#[cfg(test)]
mod parser_tests {
    use compiler::{Token, Text, Variable, OTag, CTag, Raw, Partial};
    use parser;
    use parser::{Node, Static, Value, Section, Unescaped, Part};

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
        let expected: Vec<Node> = vec![Value("token", "{{ token }}")];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_section() {
        let tokens: Vec<Token> = vec![OTag("section", false, "{{# section }}"), Variable("child_tag", "{{ child_tag }}"), CTag("section", "{{/ section }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Section("section", vec![Value("child_tag", "{{ child_tag }}")], false, "{{# section }}", "{{/ section }}")];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_inverted() {
        let tokens: Vec<Token> = vec![OTag("inverted", true, "{{^ inverted }}"), Variable("child_tag", "{{ child_tag }}"), CTag("inverted", "{{/ inverted }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Section("inverted", vec![Value("child_tag", "{{ child_tag }}")], true, "{{^ inverted }}", "{{/ inverted }}")];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn parse_unescaped() {
        let tokens: Vec<Token> = vec![Raw("unescaped", "{{& unescaped }}")];
        let nodes = parser::parse_nodes(&tokens);
        let expected: Vec<Node> = vec![Unescaped("unescaped", "{{& unescaped }}")];
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
        let value_node = Value("token", "{{ token }}");
        let section_node = Section("section", vec![Value("child_tag", "{{ child_tag }}")], false, "{{# section }}", "{{/ section }}");
        let file_node = Part("new", "{{> new }}");
        let undescaped_node = Unescaped("unescaped", "{{& unescaped }}");
        let expected: Vec<Node> = vec![static_node, value_node, section_node, file_node, undescaped_node];
        assert_eq!(nodes, expected);
    }
}
