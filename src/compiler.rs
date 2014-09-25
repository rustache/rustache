// The compiler compiles any template file into a list of
// parser usable tokens

#[deriving(Show, PartialEq, Eq)]
pub enum Token {
    Text(&'static str),
    Variable(&'static str),
    OTag(&'static str, bool), // bool denotes whether it is an inverted section tag
    CTag(&'static str),
    Raw(&'static str),
    Partial(&'static str)
}

pub struct Compiler<'a> {
    pub contents: &'static str,
    pub tokens: Vec<Token>
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

    fn find_end<I: Iterator<(uint, char)>>(&self, iter: &mut I) -> uint {
        loop {
            match iter.next() {
                Some((i, ch)) => { 
                    if self.contents.char_at(i+1) == '}' {
                        if ch == '}' {  
                            return i;
                        }
                    }
                },
                None => { break; }
            };
        }

        let (i,_) = iter.last().unwrap();

        i
    }
    fn create_tokens(&mut self) {
        let mut open_pos = 0u;
        let mut end_pos = 0u;
        let len = self.contents.len();
        let mut iter = self.contents.chars().enumerate().peekable();
        
        loop {
            match iter.next() {
                Some((i, ch)) => { 
                    if !iter.peek().is_none() {
                        match (ch, iter.peek().unwrap()) {
                            ('{', &(i, next_ch)) => {
                                open_pos = i;
                                if next_ch == '{' {
                                    match self.contents.char_at(i+1) {
                                        // unescaped literal
                                        '{' => {
                                            end_pos = self.find_end(&mut iter);
                                            self.tokens.push(Raw(self.contents.slice(open_pos + 2, end_pos).trim()));
                                        },
                                        // unescaped
                                        '&' => {
                                            end_pos = self.find_end(&mut iter);
                                            self.tokens.push(Raw(self.contents.slice(open_pos + 2, end_pos).trim()));
                                        },
                                        // section OTag
                                        '#' => {
                                            end_pos = self.find_end(&mut iter);
                                            self.tokens.push(OTag(self.contents.slice(open_pos + 2, end_pos).trim(), false));
                                        },
                                        // inverted section
                                        '^' => {
                                            end_pos = self.find_end(&mut iter);
                                            self.tokens.push(OTag(self.contents.slice(open_pos + 2, end_pos).trim(), true));
                                        },
                                        // section CTag
                                        '/' => {
                                            end_pos = self.find_end(&mut iter);
                                            self.tokens.push(CTag(self.contents.slice(open_pos + 2, end_pos).trim()));
                                        },
                                        // partial
                                        // '>' => {},
                                        '!' => {
                                            end_pos = self.find_end(&mut iter);
                                        },
                                        // variables
                                        _ => {
                                            end_pos = self.find_end(&mut iter);
                                            self.tokens.push(Variable(self.contents.slice(open_pos + 1, end_pos).trim()));   
                                        }
                                    }
                                }
                            },
                            _ => {}
                        };
                    }
                },
                None => { break; }
            }
        }
    }
}

#[test]
fn basic_compiler_test() {
    let contents = "this is some static text {{{ token }}}   
                    more static text  {{&token}}} {{! comment }}
                    {{! this is a comment}} {{#section_start}} {{^ derp }} 
                    {{/ section_end }} {{/ section_end }} {{ just_a_var}} {{ token }}   
                    more static text ";

    let mut compiler = Compiler::new(contents);

    let expected = vec![ Raw("token"), 
                         Raw("token"), 
                         OTag("section_start", false),
                         OTag("derp", true),
                         CTag("section_end"),
                         CTag("section_end"),
                         Variable("just_a_var"),Variable("token") 
                        ];

    assert_eq!(expected, compiler.tokens);
}
