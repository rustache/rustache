//! A simple parser for parsing rustache files.
//!
//! Can parse parse opening and closing rustaches and text nodes.

use std::collections::hashmap::HashMap;

// Parse an HTML template and returns a TagMap
pub fn parse(source: String) -> HashMap {
    let tag_map: HashMap<String, String> = HashMap::new();
    tag_map.insert(source);
    tag_map
}

struct Parser {
    pos: uint,
    input: String
}

impl Parser {

    // Read the next character without consuming it
    fn next_char(&self) -> char {
        self.input.as_slice().char_at(self.pos)
    }

    // Do the next characters start with a given string?
    fn starts_with(&self, string: &str) -> bool {
        self.input.as_slice().slice_from(self.pos).starts_with(string)
    }

    // Return true if all input is consumed 
    fn is_all_input_consumed(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return the current character and advance to the next character
    fn consume_char(&mut self) -> char {
        let range = self.input.as_slice().char_range_at(self.pos);
        self.pos = range.next;
        range.ch
    }

    // Consume characters until `test` returns false
    fn consume_while(&mut self, test: |char| -> bool) -> String {
        let mut result = String::new();
        while !self.is_all_input_consumed() && test(self.next_char()) {
            result.push_char(self.consume_char());
        }
        result
    }

    // Consume and discard zero or more whitespace characters 
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    // Parse a single string tag
    fn parse_string_tag(&mut self) -> String {
        match self.next_char() {
            "{{" => self.parse_element(),
            _    => self.parse_text() 
        }
    }
}

