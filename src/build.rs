pub use std::collections::{HashSet, HashMap};

use super::{Data, Static, Bool, Vector, Map};

pub struct Builder<'a> {
    data: HashMap<String, Data<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Builder<'a> {
        Builder {
            data: HashMap::new()
        }
    }

    pub fn normalize_data_map<'a>(tags: HashMap<&'a str, Data<'a>>, data: HashMap<&'a str, Data<'a>>) -> HashMap<&'a str, Data<'a>> {
        let mut value_map = HashMap::new();

        for tag in tags.into_iter() {
            let datum = data.find_equiv(&tag.as_slice())  // -> Option<'a V>
            // if data.find_equiv() finds Some, we'll want to perform a match on the returned Data
            // if data.find_equiv() finds None, return a default String
                            .unwrap_or(&Static("".to_string()))
                            .to_string();
            match datum {
                Static => ,
                Bool   => ,
                Vector => ,
                Map    =>
            }
        }

        value_map
    }

}
