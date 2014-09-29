// The compiler compiles any template file into a list of
// parser usable tokens

#[deriving(Show, PartialEq, Eq)]
pub enum Token<'a> {
    Text(&'a str),
    Variable(&'a str, &'a str),
    OTag(&'a str, bool, &'a str), // bool denotes whether it is an inverted section tag
    CTag(&'a str, &'a str),
    Raw(&'a str, &'a str),
    Partial(&'a str, &'a str)
}

pub struct Compiler<'a> {
    pub contents: &'a str,
    pub tokens: Vec<Token<'a>>
}

impl<'a> Compiler<'a> {
    // Compiler takes in the context of a file/string to compile into tokens
    pub fn new(contents: &'a str) -> Compiler {
        let mut compiler = Compiler {
            contents: contents,
            tokens: vec![]
        };

        compiler.create_tokens();
        compiler
    }

    fn create_tokens(&mut self) {
        let mut open_pos = 0u;
        let mut close_pos = 0u;
        let len = self.contents.len();

        for (mut i, c) in self.contents.chars().enumerate() {
            if c == '{' && self.contents.char_at(i+1) == '{' {
                open_pos = i;
                if open_pos != close_pos {
                    self.tokens.push(Text(self.contents.slice(close_pos, open_pos)));
                }
            }
            if c == '}' && i < len - 1 && self.contents.char_at(i+1) == '}' && self.contents.char_at(open_pos) == '{'{
                close_pos = i + 2;
                let raw = self.contents.slice(open_pos, close_pos);
                let val = self.contents.slice(open_pos + 2, close_pos - 2);
                match val.char_at(0) {
                    '!' => continue, // comment, skip over
                    '#' => self.tokens.push(OTag(val.slice_from(1).trim(), false, raw)), // Section OTAG
                    '/' => self.tokens.push(CTag(val.slice_from(1).trim(), raw)), // Section CTAG
                    '^' => self.tokens.push(OTag(val.slice_from(1).trim(), true, raw)), // Inverted Section
                    '>' => self.tokens.push(Partial(val.slice_from(1).trim(), raw)), // partial
                    '&' => self.tokens.push(Raw(val.slice_from(1).trim(), raw)), // Unescaped
                    '{' => continue, // unescaped literal
                    _   => self.tokens.push(Variable(val.trim(), raw))

                }
            }
        }
        if close_pos < len { 
            self.tokens.push(Text(self.contents.slice_from(close_pos)));
        }
    }
}

#[test]
fn basic_compiler_test() {
    let contents = "<div> <h1> {{ token }} {{> partial }} </h1> </div>";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("<div> <h1> "), 
                        Variable("token", "{{ token }}"),
                        Text(" "), 
                        Partial("partial", "{{> partial }}"), Text(" </h1> </div>")];

    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_all_directives() {
    let contents = "{{!comment}}{{#section}}{{/section}}{{^isection}}{{/isection}}{{>partial}}{{&unescaped}}{{value}}other crap";
    let compiler = Compiler::new(contents);
    let expected = vec![OTag("section", false, "{{#section}}"), 
                        CTag("section", "{{/section}}"),
                        OTag("isection", true, "{{^isection}}"), 
                        CTag("isection", "{{/isection}}"), 
                        Partial("partial", "{{>partial}}"),
                        Raw("unescaped", "{{&unescaped}}"),
                        Variable("value", "{{value}}"),
                        Text("other crap")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_missing_close_on_comment() {
    let contents = "{{!comment";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("{{!comment")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_working_comment() {
    let contents = "{{!comment}}";
    let compiler = Compiler::new(contents);
    let expected = vec![];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_missing_close_on_section_close() {
    let contents = "{{#section}}{{/section";
    let compiler = Compiler::new(contents);
    let expected = vec![OTag("section", false, "{{#section}}"), Text("{{/section")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_working_section() {
    let contents = "{{#section}}{{/section}}";
    let compiler = Compiler::new(contents);
    let expected = vec![OTag("section", false, "{{#section}}"), CTag("section", "{{/section}}")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_missing_close_on_inverted_section_close() {
    let contents = "{{^isection}}{{/isection";
    let compiler = Compiler::new(contents);
    let expected = vec![OTag("isection", true, "{{^isection}}"), Text("{{/isection")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_missing_close_on_partial() {
    let contents = "{{>partial";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("{{>partial")];
    assert_eq!(expected, compiler.tokens);    
}

#[test]
fn test_working_partial() {
    let contents = "{{>partial}}";
    let compiler = Compiler::new(contents);
    let expected = vec![Partial("partial", "{{>partial}}")];
    assert_eq!(expected, compiler.tokens);    
}

#[test]
fn test_missing_close_on_unescaped() {
    let contents = "{{&unescaped";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("{{&unescaped")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_working_unescape() {
    let contents = "{{&unescaped}}";
    let compiler = Compiler::new(contents);
    let expected = vec![Raw("unescaped", "{{&unescaped}}")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_missing_close_on_partial_plus_unescaped() {
    let contents = "{{>partial}}{{&unescaped";
    let compiler = Compiler::new(contents);
    let expected = vec![Partial("partial", "{{>partial}}"), Text("{{&unescaped")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_missing_close_on_value() {
    let contents = "{{value other crap";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("{{value other crap")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_bad_opens() {
    let contents = "value}} other crap";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("value}} other crap")];
    assert_eq!(expected, compiler.tokens);
}

#[test]
fn test_single_brace_open() {
    let contents = "{value other crap";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("{value other crap")];
    assert_eq!(expected, compiler.tokens);    
}

#[test]
fn test_single_brace_close() {
    let contents = "value} other crap";
    let compiler = Compiler::new(contents);
    let expected = vec![Text("value} other crap")];
    assert_eq!(expected, compiler.tokens);    
}
