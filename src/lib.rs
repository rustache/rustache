use std::io::{File, FileNotFound};
use std::str;
// helpers



fn get_template(template_path: &str) -> {
	let path = Path::new(template_path);

    match File::open(&path).read_to_end() {
        Ok(contents) => {
            let string = match str::from_utf8(contents.as_slice()) {
                Some(string) => string.to_string(),
                None => { fail!("Could not parse file as UTF-8"); }
            };

            string
        },
        Err(e) => {
            if e.kind == FileNotFound {
                fail!("failed to read file {}", path.display());
            } else {
                fail!("error reading file: {}", e);
            }
        }
    }
}

#[test]
fn should_retrieve_file() {
	let path: &str = "./test_templates/sample.html";

	get_template(path);

}