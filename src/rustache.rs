pub mod Rustache {
    use std::io::File;
    use serialize::{json};
    use serialize::json::{Json, Boolean, Null, I64, U64, F64, String, List, Object};
    use build::{HashBuilder, VecBuilder};
    use template::Template;
    use compiler::Compiler;
    use parser::Parser;

    pub fn render<'a, W: Writer>(path: &str, data: &HashBuilder, writer: &mut W) {
        let file = read_file(Path::new(path));
        let compiler = Compiler::new(file.as_slice());
        let parser = Parser::new(&compiler.tokens);
        Template::new().render_data(writer, data, &parser);
    }

    pub fn render_text<'a, W: Writer>(input: &'a str, data: &HashBuilder, writer: &mut W) {
        let compiler = Compiler::new(input);
        let parser = Parser::new(&compiler.tokens);
        Template::new().render_data(writer, data, &parser);
    }

    pub fn render_json<'a, W: Writer>(template_path: &str, data_path: &str, writer: &mut W) {
        let data_string = read_file(Path::new(data_path));

        let json = match json::from_str(data_string.as_slice()) {
            Ok(json) => json,
            Err(err) => fail!("Invalid JSON. {}", err)
        };
        
        let data = parse_json(&json);

        println!("{}", data);

        render(template_path, &data, writer);
    }

    fn parse_json(json: &Json) -> HashBuilder{
        let mut data = HashBuilder::new();
        for (k, v) in json.as_object().unwrap().iter() {
            match v {
                &I64(num) => {
                    data = data.insert_string(k.as_slice(), num.to_string());
                }
                &U64(num) => {
                    data = data.insert_string(k.as_slice(), num.to_string());
                },
                &F64(num) => {
                    data = data.insert_string(k.as_slice(), num.to_string());
                },
                &Boolean(val) => {
                    data = data.insert_bool(k.as_slice(), val);
                },
                &List(ref list) => {
                    data = data.insert_vector(k.as_slice(), |mut builder| {
                        for item in list.iter() {
                            if item.is_object() {
                                builder = builder.push_hash(|builder| {
                                    parse_json(item)
                                });
                            } else if item.is_list() {
                                let vec = parse_json(item);
                                builder = builder.push_vector(|vector| {
                                    vector
                                });
                            } else if item.is_string() {
                                builder = builder.push_string(item.as_string().unwrap());
                            } else if item.is_boolean() {
                                builder = builder.push_bool(item.as_boolean().unwrap());
                            }
                        }
                        builder
                    });
                },
                &Object(ref obj) => {
                    data = data.insert_hash(k.as_slice(), |builder| {
                        parse_json(v)
                    });
                },
                &Null => {},
                &String(ref text) => {
                    data = data.insert_string(k.as_slice(), text.as_slice());
                },
            }
        }

        data
    }

    pub fn read_file(path: Path) -> String {
        // Open the file path
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => fail!("{} {}",display ,why.desc),
            Ok(file) => file,
        };

        // Read the file contents into a heap allocated string
        let contents = match file.read_to_string() {
            Err(why) => fail!("{}", why.desc),
            Ok(text) => text,
        };

        contents
    }
}

