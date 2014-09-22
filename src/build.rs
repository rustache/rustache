pub use std::collections::hashmap::HashSet;
pub use std::collections::hashmap::HashMap;

pub struct Build<'a>;

impl<'a> Build<'a> {
    pub fn new() -> Build<'a> {
        Build
    }
    pub fn create_data_map<'a>(tags: HashSet<String>, data: HashMap<&'a str, &'a str>) -> HashMap<String, String> {
        let mut value_map: HashMap<String, String> = HashMap::new();
        for &tag in tags.iter() {
            if data.contains_key(&tag.as_slice()) {
                value_map.insert(tag, data[tag.as_slice()].to_string());
            } else {
                value_map.insert(tag, "".to_string());
            }
        }
        value_map
    }
}
