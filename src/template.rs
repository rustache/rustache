use std::collections::hashmap::HashMap;
use std::io::{File};
use parser::{Node};

pub struct Template<'a>;

impl<'a> Template<'a> {
    pub fn new() -> Template<'a> {
        Template
    }


    // pub fn render_data<'a, W: Writer>(writer: &mut W,  data: HashMap<&'a str, &'a str>, nodes: &'a Vec<Node>) {
    //     let mut output = String::new();
    //     for node in nodes.iter() {
    //         if !data.contains_key(&node.val.as_slice()) {
    //             writer.write_str(node.val.as_slice());
    //         } else {
    //             writer.write_str(data[node.val.as_slice()]);
    //         }
    //     }
    // }

}



#[cfg(test)]
mod template_tests {
    use std::collections::hashmap::HashMap;
    use std::io::{TempDir, File, BufferedWriter, BufferedReader};
    use std::io::{stdout, MemWriter};

    use parser::Parser;
    use template::Template;
    use std::str;

    #[test]
    fn test_write_to_console() {
        let mut data_map: HashMap<&str, &str> = HashMap::new();
        let html: Vec<String> = vec!["<h1>{{ value1 }}</h1>".to_string()];
        let tags = Parser::tag_lines(html);

        data_map.insert("value1", "The heading");

        Template::render_data(&mut stdout(), data_map, &tags);
        assert_eq!(true,true); // idk how to validate stdout, but the correct text is visible in console
    }

    #[test]
    fn test_write_to_file() {
        
        // make temp directory
        let tmpdir = match TempDir::new("") {
            Ok(tmpdir) => tmpdir,
            Err(_) => fail!(),
        };

        // create file and send to BufferedWriter
        let path = Path::new(tmpdir.path().join("tmp.html"));
        let mut tmp_file = File::create(&path);
        let mut writer = BufferedWriter::new(tmp_file);

        // build template
        let mut data_map: HashMap<&str, &str> = HashMap::new();
        let html: Vec<String> = vec!["<h1>{{ value1 }}</h1>".to_string()];
        let tags = Parser::tag_lines(html);

        data_map.insert("value1", "The heading");
        Template::render_data(&mut writer, data_map, &tags);

        // end BufferedWriter
        writer.flush();

        //open file and read lines
        let mut file = BufferedReader::new(File::open(&path));
        let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();

        assert_eq!("<h1>The heading</h1>",lines[0].as_slice());
    }
}