pub use std::collections::{HashSet, HashMap};

use super::{Data, Static, Bool, Vector, Map};

/// `MapBuilder` is a helper type that constructs `Data` types in a HashMap
pub struct MapBuilder<'a> {
    data: HashMap<String, Data<'a>>
}

impl<'a> MapBuilder<'a> {
    pub fn new() -> MapBuilder<'a> {
        MapBuilder {
            data: HashMap::new()
        }
    }

    pub fn insert_static<K: StrAllocating, V: StrAllocating>(self, key: K, value: V) -> MapBuilder<'a> {
        let MapBuilder { mut data } = self;
        data.insert(key.into_string(), Static(value.into_string()));
        MapBuilder { data: data }
    }

    pub fn insert_bool<K: StrAllocating>(self, key: K, value: bool) -> MapBuilder<'a> {
        let MapBuilder { mut data } = self;
        data.insert(key.into_string(), Bool(value));
        MapBuilder { data: data }
    }

    pub fn insert_vec<K: StrAllocating>(self, key: K, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> MapBuilder<'a> {
        let MapBuilder { mut data } = self;
        let builder = f(VecBuilder::new());
        data.insert(key.into_string(), builder.build());
        MapBuilder { data: data }
    }  

    pub fn insert_map<K: StrAllocating>(self, key: K, f: |MapBuilder<'a>| -> MapBuilder<'a>) -> MapBuilder<'a> {
        let MapBuilder { mut data } = self;
        let builder = f(MapBuilder::new());
        data.insert(key.into_string(), builder.build());
        MapBuilder { data: data }
    }

    /// Return the built `Data`
    pub fn build(self) -> Data<'a> {
        Map(self.data)
    }

}

/// `VecBuilder` is a helper type that constructs `Data` types in a Vector
pub struct VecBuilder<'a> {
    data: Vec<Data<'a>>
}

impl<'a> VecBuilder<'a> {
    pub fn new() -> VecBuilder<'a> {
        VecBuilder {
            data: Vec::new()
        }
    }

    pub fn push_static<T: StrAllocating>(self, value: T) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Static(value.into_string()));
        VecBuilder { data: data }
    }

    pub fn push_bool(self, value: bool) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Bool(value));
        VecBuilder { data: data }
    }

    pub fn push_vec(self, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(VecBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    pub fn push_map(self, f: |MapBuilder<'a>| -> MapBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(MapBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    pub fn build(self) -> Data<'a> {
        Vector(self.data)
    }
}


/*    pub fn normalize_data_map<'a>(tags: HashMap<&'a str, Data<'a>>, data: HashMap<&'a str, Data<'a>>) -> HashMap<&'a str, Data<'a>> {
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
    }*/
