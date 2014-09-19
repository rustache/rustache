#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::io::File;

// Helpers
fn get_template(template_path: &str) -> String {
    let path = Path::new(template_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => fail!("Couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };

    let template_str: String = match file.read_to_string() {
        Err(why)   => fail!("Couldn't read {}: {}", display, why.desc),
        Ok(string) =>  string,
    };

    template_str
}

// Capture all regex matches for rustache tags and return them as a vector of
// string slices.  Results will be used by the parser in order to create the
// TagMap.
fn find_tag_matches(input: &str) -> Vec<&str>{
    let mut result: Vec<&str> = Vec::new();
    let re = regex!(r"(\{\{\s?[\w\s]*\s?\}\})");
    
    for cap in re.captures_iter(input) {
        result.push(cap.at(1));
    }

    result
}

#[test]
fn should_retrieve_file() {
    let path = "src/test_templates/sample.html";
    let expected = String::new();

    assert_eq!(expected.append("<div>"), get_template(path));
}

#[test]
fn test_bucketing() {
    let test_string: &str = "{{variable1}},{{variable2}},{{variable3}}";
    let expected: Vec<&str> = vec!["{{variable1}}","{{variable2}}","{{variable3}}"];
    let result = find_tag_matches(test_string);
    assert_eq!(result, expected);
}