// The compiler tokenizes any template file or string into a list of
// parser usable tokens

#[deriving(Show, PartialEq, Eq)]
pub enum Token<'a> {
    Text(&'a str),
    Variable(&'a str, &'a str),
    OTag(&'a str, bool, &'a str), // bool denotes whether it is an inverted section tag
    CTag(&'a str, &'a str),
    Raw(&'a str, &'a str),
    Partial(&'a str, &'a str),
}

pub fn create_tokens<'a>(contents: &'a str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = Vec::new();
    let re = regex!(r"\{\{(\{?\S?\s*?[\w\.\s]*.*?\s*?\}?)\}\}");
    let len = contents.len();
    let mut close_pos = 0u;
    for cap in re.captures_iter(contents) {
        let inner = cap.at(1);
        let outer = cap.at(0);
        let (o, c) = cap.pos(0).unwrap();
        if o != close_pos {
            tokens.push(Text(contents.slice(close_pos, o)));
        }

        close_pos = c;
        match inner.char_at(0) {
            '!' => continue,
            '#' => tokens.push(OTag(inner.slice_from(1).trim(), false, outer)),
            '/' => tokens.push(CTag(inner.slice_from(1).trim(), outer)),
            '^' => tokens.push(OTag(inner.slice_from(1).trim(), true, outer)),
            '>' => tokens.push(Partial(inner.slice_from(1).trim(), outer)),
            '&' => tokens.push(Raw(inner.slice_from(1).trim(), outer)),
            '{' => tokens.push(Raw(inner.slice(1, inner.len() - 1).trim(), outer)),
            _   => tokens.push(Variable(inner.trim(), outer))
        }
    }
    if close_pos < len { 
        tokens.push(Text(contents.slice_from(close_pos)));
    }
    tokens
}

#[cfg(test)]
mod compiler_tests {
    use compiler;
    use compiler::{Text, Variable, OTag, CTag, Raw, Partial};

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
        let expected = vec![OTag("section", false, "{{#section}}"), 
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
        let expected = vec![];
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

