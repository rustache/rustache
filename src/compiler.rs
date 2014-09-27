// The compiler compiles any template file into a list of
// parser usable tokens

#[deriving(Show, PartialEq, Eq)]
pub enum Token<'a> {
    Text(&'a str),
    Variable(&'a str),
    OTag(&'a str, bool), // bool denotes whether it is an inverted section tag
    CTag(&'a str),
    Raw(&'a str),
    Partial(&'a str)
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
            if c == '}' && i < len - 1 && self.contents.char_at(i+1) == '}' {
                close_pos = i + 2;
                let val = self.contents.slice(open_pos + 2, close_pos - 2);
                match val.char_at(0) {
                    '!' => continue, // comment, skip over
                    '#' => self.tokens.push(OTag(val.slice_from(1).trim(), false)), // Section OTAG
                    '/' => self.tokens.push(CTag(val.slice_from(1).trim())), // Section CTAG
                    '^' => self.tokens.push(OTag(val.slice_from(1).trim(), true)), // Inverted Section
                    '>' => self.tokens.push(Partial(val.slice_from(1).trim())), // partial
                    '&' => self.tokens.push(Raw(val.slice_from(1).trim())), // Unescaped
                    '{' => continue, // unescaped literal
                    _   => self.tokens.push(Variable(val.trim()))

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
    let mut compiler = Compiler::new(contents);
    let expected = vec![Text("<div> <h1> "), Variable("token"), Text(" "), Partial("partial"), Text(" </h1> </div>")];

    assert_eq!(expected, compiler.tokens);
}
