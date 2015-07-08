extern crate regex;

use self::regex::Regex;
use self::Token::*;

// The compiler takes in a stringified template file or a string and
// splits into a list of tokens to be processed by the parser.

// Token represents the basic data source for different sections of
// text provided within the template.  Raw tag values are stored
// for use in lambdas.

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Text(&'a str), // (text)
    Variable(&'a str, &'a str), // (name, tag)
    OTag(&'a str, bool, &'a str), // (name, inverted, tag, whitespace)
    CTag(&'a str, &'a str), // (name, tag, whitespace)
    Raw(&'a str, &'a str), // (name, tag)
    Partial(&'a str, &'a str), // (name, tag)
    Comment
}

// Entry point to the template compiler. It compiles a token list of
// all applicable tags within a template to send to the parser.
pub fn create_tokens<'a>(contents: &'a str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = Vec::new();

    // Close position and length are used to catch trailing characters afer last
    // tag capture, or if no tags are present in the template.
    let mut close_pos = 0;
    let len = contents.len();

    // (text)(whitespace)( (tag) )(whitespace)
    let re = Regex::new(r"(.*?)([ \t\r\n]*)(\{\{(\{?\S?\s*?[\w\.\s]*.*?\s*?\}?)\}\})([ \t\r\n]*)").unwrap();

    // Grab all captures and process
    for cap in re.captures_iter(contents) {
        // Establish groups for tag capture, preventing lookup for each call
        let preceding_text = cap.at(1).unwrap_or("");
        let preceding_whitespace = cap.at(2).unwrap_or("");
        let outer = cap.at(3).unwrap_or("");
        let inner = cap.at(4).unwrap_or("");
        let trailing_whitespace = cap.at(5).unwrap_or("");

        // Grab closing index
        let (_, c) = cap.pos(0).unwrap();

        // Catch preceding text
        if !preceding_text.is_empty() {
            tokens.push(Text(preceding_text));
        }

        // Catch preceding whitespace
        if !preceding_whitespace.is_empty() {
            tokens.push(Text(preceding_whitespace));
        }

        // Advance last closing position and add captured token
        close_pos = c;
        add_token(inner, outer, &mut tokens);

        // Catch trailing whitespace
        if !trailing_whitespace.is_empty() {
            tokens.push(Text(&trailing_whitespace));
        }
    }

    // Catch trailing text
    if close_pos < len {
        tokens.push(Text(&contents[close_pos..]));
    }

    // Return
    tokens
}

// Simple method for categorizing and adding appropriate token
fn add_token<'a>(inner: &'a str, outer: &'a str, tokens: &mut Vec<Token<'a>>) {
    match &inner[0..1] {
        "!" => tokens.push(Comment),
        "#" => tokens.push(OTag(inner[1..].trim(), false, outer)),
        "/" => tokens.push(CTag(inner[1..].trim(), outer)),
        "^" => tokens.push(OTag(inner[1..].trim(), true, outer)),
        ">" => tokens.push(Partial(inner[1..].trim(), outer)),
        "&" => tokens.push(Raw(inner[1..].trim(), outer)),
        "{" => tokens.push(Raw(inner[1 .. inner.len() - 1].trim(), outer)),
        _   => tokens.push(Variable(inner.trim(), outer))
    }
}

#[cfg(test)]
mod compiler_tests {
    use compiler;
    use compiler::{Text, Variable, OTag, CTag, Raw, Partial, Comment};

    #[test]
    fn test_one_char() {
        let contents = "c";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("c")];

        assert_eq!(expected, tokens);
    }

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
        let expected = vec![Text("<div> <h1>"),
                            Text(" "),
                            Variable("token", "{{ token }}"),
                            Text(" "),
                            Raw("unescaped", "{{{ unescaped }}}"),
                            Text(" "),
                            Partial("partial", "{{> partial }}"),
                            Text(" "),
                            Text("</h1> </div>")
                            ];

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
    fn test_embedded_comment() {
        let contents = "text {{!comment}} text";
        let tokens = compiler::create_tokens(contents);
        let expected = vec![Text("text"),
                            Text(" "),
                            Comment,
                            Text(" "),
                            Text("text"),
                            ];
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
