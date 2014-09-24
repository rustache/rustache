// The compiler compiles any template file into a list of
// parser usable tokens

#[deriving(Show, PartialEq, Eq)]
pub enum Token {
    Text(&'static str),
    Variable(&'static str),
    OTag(&'static str, bool), // bool denotes whether it is an inverted section tag
    CTag(&'static str),
    Raw(&'static str),
}

pub struct Compiler<'a> {
    contents: &'static str,
    tokens: Vec<Token>
}

impl<'a> Compiler<'a> {
    // Compiler takes in the context of a file/string to compile into tokens
    pub fn new(contents: &'static str) -> Compiler {
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
                i += 1;
            }
            if c == '}' && i < len - 1 && self.contents.char_at(i+1) == '}' {
                close_pos = i + 2;
                let val = self.contents.slice(open_pos + 2u, close_pos - 2u);
                match val.char_at(0) {
                    '!' => continue, // comment, skip over
                    '#' => self.tokens.push(OTag(val.slice_from(1).trim(), false)), // Section OTAG
                    '/' => self.tokens.push(CTag(val.slice_from(1).trim())), // Section CTAG
                    '^' => self.tokens.push(OTag(val.slice_from(1).trim(), true)), // Inverted Section
                    '>' => continue, // partial
                    '&' => self.tokens.push(Raw(val.slice_from(1).trim())), // Unescaped
                    '{' => continue, // unescaped literal
                    _ => self.tokens.push(Variable(val.trim()))

                }
                i += 2;
            }
            
        }
        if close_pos < len {
            self.tokens.push(Text(self.contents.slice_from(close_pos)));
        }
    }
}

#[test]
fn basic_compiler_test() {
    let contents = "Static String {{ token }}";
    let compiler = Compiler::new(contents);
    let static_token = Static("Static String ");
    let value_token = Value("token");
    let expected = vec![static_token, value_token];
    assert_eq!(expected, compiler.tokens);
}



