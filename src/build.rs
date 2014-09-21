pub use std::collections::hashmap::HashSet;
pub use std::collections::hashmap::HashMap;

pub struct Build;

impl Build {
    pub fn create_data_map<'a>(tags: HashSet<&'a str>, data: HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
        let mut value_map: HashMap<&str, &str> = HashMap::new();
        for &tag in tags.iter() {
            value_map.insert(tag, data[tag]);
        }
        value_map
    }
}
