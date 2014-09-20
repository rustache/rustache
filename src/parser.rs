//! A simple parser for parsing rustache files.
//!
//! Can parse parse opening and closing rustaches and text nodes.

use std::collections::hashmap::HashMap;

struct Token<'a> {
    pos: uint,
    name: &'a str,
    tag_type: Tag<'a>,
    data: &'a str
}

impl<'a> Token<'a> {
    fn new(pos: uint, name: &'a str, tag_type: Tag) -> Token<'a> {
        Token {
            pos: pos,
            name: name,
            tag_type: tag_type,
            data: ""
        }
    }
}

 pub enum Tag<'a> {
    Unescaped,
    Inverted,
    Comment,
    Partial,
    Section,
}

pub struct Parser<'a> {
    pub input: String,
    pub token_map: HashMap<String, Vec<Token<'a>>>
}

impl<'a> Parser<'a> {
    fn new(input: String) -> Parser<'a> {
        Parser {
            input: input
        }
    }

    // Parse a single string tag
    fn parse_string_tag<'a >(input: &str, token: &Token) -> Vec<Token<'a>> {
        let mut result: Vec<Token> = Vec::new();
        let tokens = Parser::find_token_matches(input);
        for token in tokens {
            let (pos, name, tag_type) = token;
            Token::new(pos, name, tag_type);
        }
        result
    }

    pub fn parse<'a>(source: Vec<Token>) -> HashMap<&str, Vec<Token<'a>>> {
        let tag_map: HashMap<&str, Vec<Token>> = HashMap::new();
        for token in source.iter() {
            tag_map.insert(token.name, token);
        }
        tag_map
    }

    pub fn insert_item_at_key(&self, key: Token<'a>) {
        self.token_map.insert()
    }


    // Capture all regex matches for mustache tags and return them as a vector of
    // tuples containing position, name and tagtype. Results will be used by the 
    // to create the TokenMap.
    fn find_token_matches(input: &str) -> Vec<(uint, &str, &str)>{
        let mut result = Vec::new();
        let re = regex!(r"(\{\{*\S?\s?[\w\s]*\s?\S?\}\})");
        for cap in re.captures_iter(input) {
            let (start, end) = cap.pos(1).unwrap();
            let mut name = cap.at(1);
            let mut tag_type  = match name.char_at(2) {
                '&' => Unescaped,
                '{' => Unescaped,
                '!' => Comment,
                '>' => Partial,
                '#' => Section,
                '^' => Inverted,
                _   => Partial
            };

            result.push((start, name, tag_type));
        }

        result
    }


}

// #[test]
// fn test_token_matches() {
//     let test_string: &str = "{{variable1}},{{variable2}},{{variable3}}";
//     let expected: Vec<&str> = vec!["{{variable1}}","{{variable2}}","{{variable3}}"];
//     let result = find_tag_matches(test_string);
//     assert_eq!(result, expected);
// }
