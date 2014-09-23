pub use std::collections::{HashSet, HashMap};

pub struct Build<'a>;

impl<'a> Build<'a> {
    pub fn new() -> Build<'a> {
        Build
    }

    pub fn create_data_map<'a>(tags: HashSet<String>, data: HashMap<&'a str, &'a str>) -> HashMap<String, String> {
        let mut value_map = HashMap::new();

        for tag in tags.into_iter() {
            let datum = data.find_equiv(&tag.as_slice())
                .unwrap_or(&"")
                .to_string();
            value_map.insert(tag, datum);
        }

        value_map
    }
}
