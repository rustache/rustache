// The compiler takes in a stringified template file or a string and
// splits into a list of tokens to be processed by the parser.

// Token represents the basic data source for different sections of
// text provided within the template.  Raw tag values are stored
// for use in lambdas.
#[deriving(Show, PartialEq, Eq)]
pub enum Token<'a> {
    Text(&'a str), // (text)
    Comment, // no data stored, will be passed over by the parser
    Variable(&'a str, &'a str), // (name, tag)
    OTag(&'a str, bool, &'a str), // (name, inverted, tag, whitespace)
    CTag(&'a str, &'a str), // (name, tag, whitespace)
    Raw(&'a str, &'a str), // (name, tag)
    Partial(&'a str, &'a str), // (name, tag)
}

enum Status {
    TAG,
    TEXT
}

// Create_tokens is the entry point to the template compiler. It compiles a raw list of
// all applicable tags within a template to send to the parser.
pub fn create_tokens<'a>(contents: &'a str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut char = contents.chars().enumerate();
    let mut line_number = 1u;
    let mut close = 0u;
    let mut open = 0u;
    let mut status = TEXT;

    loop {
        // Advance character
        match char.next() {
            Some((i, ch)) => {
                match ch {
                    '{' => {
                        handle_otag(contents, i, &mut status, &mut open);
                    },
                    '}' => {
                        handle_ctag(contents, i, &mut status, open, &mut close, &mut tokens);
                    },
                    // Increment the line count
                    '\n' => line_number += 1,
                    _ => continue,
                }
            },
            // Reached the end of the input, handle the uncategorized text
            None => {
                if close == 0 {
                    tokens.push(Text(contents.slice_from(0)));
                } else if close != contents.len() - 1 {
                    tokens.push(Text(contents.slice_from(close + 1)));
                } 
                break;
            }
        }
    }
    tokens
}

fn handle_otag(contents: &str, i: uint, status: &mut Status, open:  &mut uint) {
    // Ensure not to check an out of bounds index
    if i < contents.len() - 1 {
        match contents.char_at(i + 1) {
            '{' => {
                match *status {
                    TAG => {
                        // Account for triple tags, without duplicate entries
                        if i == *open + 1 { return; }

                        // Reset the opening point in the event of an erroneous entry
                        *open = i;
                    },
                    TEXT => {
                        *status = TAG;
                        *open = i;
                    },
                }
            },
            // Not an opening tag
            _ => return
        }
    }
}

fn handle_ctag<'a>(contents: &'a str, i: uint, status: &mut Status,  open: uint, close: &mut uint, tokens: &mut Vec<Token<'a>>) {
    // Ensure not to try and index out of bounds
    if contents.len() - i > 1 {
        match contents.char_at(i + 1) {
            '}' => {
                match *status {
                    TAG => {
                        // If currently in a tag, ensure that the open and close positions are not equal
                        // before adding a text token
                        if open - *close >= 1 {
                            match *close {
                                0 => {
                                    tokens.push(Text(contents.slice_to(open)))
                                },
                                _ => {
                                    if open - *close != 1 {
                                        tokens.push(Text(contents.slice(*close + 1, open)));
                                    }
                                }
                            }
                        }
                        // Update closing position and change status
                        *close = i + 1;
                        *status = TEXT;
                        // Do not index outside of length
                        if contents.len() - i > 2 {
                            match contents.char_at(i + 2) {
                                // Triple tag, increment closing index
                                '}' => *close += 1,
                                _ => {},
                            }
                        }
                        tokens.push(categorize_token(contents.slice(open, *close + 1)));
                    },
                    TEXT => return,
                }
            },
            _ => return,
        }
    }
}

fn categorize_token<'a>(text: &'a str) -> Token<'a> {
    let len = text.len();
    match text.char_at(2) {
        // Handle special tags or treat as normal variables
        '!' => return Comment,
        '#' => return OTag(text.slice(3, len - 2).trim(), false, text), // Open section
        '/' => return CTag(text.slice(3, len - 2).trim(), text), // Close section
        '^' => return OTag(text.slice(3, len - 2).trim(), true, text), // Open inverted section
        '>' => return Partial(text.slice(3, len - 2).trim(), text), // Partial tag (external file)
        '&' => return Raw(text.slice(3, len - 2).trim(), text), // Unescaped tag, do not HTML escape
        '{' => return Raw(text.slice(3, len - 3).trim(), text), // Unescaped tag, do not HTML escape
        _   => return Variable(text.slice(2, len - 2).trim(), text) // Normal mustache variable
    }
}

#[cfg(test)]
mod compiler_tests {
    use compiler;
    use compiler::{Text, Variable, OTag, CTag, Raw, Partial, Comment};

    #[test]
    fn test_extended_dot_notation() {
        let contents = "{{ test.test.test.test }}";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Variable("test.test.test.test", "{{ test.test.test.test }}")];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn basic_compiler_test() {
        let contents = "<div> <h1> {{ token }} {{{ unescaped }}} {{> partial }} </h1> </div>";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("<div> <h1> "), 
                            Variable("token", "{{ token }}"),
                            Text(" "),
                            Raw("unescaped", "{{{ unescaped }}}"),
                            Text(" "),
                            Partial("partial", "{{> partial }}"), Text(" </h1> </div>")];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_all_directives() {
        let contents = "{{!comment}}{{#section}}{{/section}}{{^isection}}{{/isection}}{{>partial}}{{&unescaped}}{{value}}other crap";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Comment,
                            OTag("section", false, "{{#section}}"), 
                            CTag("section", "{{/section}}"),
                            OTag("isection", true, "{{^isection}}"), 
                            CTag("isection", "{{/isection}}"), 
                            Partial("partial", "{{>partial}}"),
                            Raw("unescaped", "{{&unescaped}}"),
                            Variable("value", "{{value}}"),
                            Text("other crap")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_missing_close_on_comment() {
        let contents = "{{!comment";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("{{!comment")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_working_comment() {
        let contents = "{{!comment}}";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Comment];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_missing_close_on_section_close() {
        let contents = "{{#section}}{{/section";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![OTag("section", false, "{{#section}}"), Text("{{/section")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_working_section() {
        let contents = "{{#section}}{{/section}}";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![OTag("section", false, "{{#section}}"), CTag("section", "{{/section}}")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_missing_close_on_inverted_section_close() {
        let contents = "{{^isection}}{{/isection";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![OTag("isection", true, "{{^isection}}"), Text("{{/isection")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_missing_close_on_partial() {
        let contents = "{{>partial";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("{{>partial")];
        assert_eq!(expected, tokens);    
    }

    #[test]
    fn test_working_partial() {
        let contents = "{{>partial}}";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Partial("partial", "{{>partial}}")];
        assert_eq!(expected, tokens);    
    }

    #[test]
    fn test_missing_close_on_unescaped() {
        let contents = "{{&unescaped";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("{{&unescaped")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_working_unescape() {
        let contents = "{{&unescaped}}";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Raw("unescaped", "{{&unescaped}}")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_missing_close_on_partial_plus_unescaped() {
        let contents = "{{>partial}}{{&unescaped";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Partial("partial", "{{>partial}}"), Text("{{&unescaped")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_missing_close_on_value() {
        let contents = "{{value other crap";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("{{value other crap")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_bad_opens() {
        let contents = "value}} other crap";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("value}} other crap")];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_single_brace_open() {
        let contents = "{value other crap";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("{value other crap")];
        assert_eq!(expected, tokens);    
    }

    #[test]
    fn test_single_brace_close() {
        let contents = "value} other crap";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("value} other crap")];
        assert_eq!(expected, tokens);    
    }
}