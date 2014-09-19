use std::io::File;

// helpers
fn get_template(template_path: &str) -> String {
    let path = Path::new(template_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => fail!("couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };

    let template_str: String = match file.read_to_string() {
        Err(why) => fail!("couldn't read {}: {}", display, why.desc),
        Ok( string) =>  string,
    };


    template_str
}

#[test]
fn should_retrieve_file() {
    let path = "src/test_templates/sample.html";
    let expected = String::new();

    assert_eq!(expected.append("<div>"), get_template(path));
}