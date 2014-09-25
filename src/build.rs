pub use std::collections::HashMap;

use super::{Data, Static, Bool, Vector, Hash};

/// `HashBuilder` is a helper type that constructs `Data` types in a HashMap
pub struct HashBuilder<'a> {
    data: HashMap<String, Data<'a>>
}

impl<'a> HashBuilder<'a> {
    pub fn new() -> HashBuilder<'a> {
        HashBuilder {
            data: HashMap::new()
        }
    }

    pub fn insert_string<K: StrAllocating, V: StrAllocating>(self, key: K, value: V) -> HashBuilder<'a> {
        let HashBuilder { mut data } = self;
        data.insert(key.into_string(), Static(value.into_string()));
        HashBuilder { data: data }
    }

    pub fn insert_bool<K: StrAllocating>(self, key: K, value: bool) -> HashBuilder<'a> {
        let HashBuilder { mut data } = self;
        data.insert(key.into_string(), Bool(value));
        HashBuilder { data: data }
    }

    pub fn insert_vector<K: StrAllocating>(self, key: K, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> HashBuilder<'a> {
        let HashBuilder { mut data } = self;
        let builder = f(VecBuilder::new());
        data.insert(key.into_string(), builder.build());
        HashBuilder { data: data }
    }  

    pub fn insert_hash<K: StrAllocating>(self, key: K, f: |HashBuilder<'a>| -> HashBuilder<'a>) -> HashBuilder<'a> {
        let HashBuilder { mut data } = self;
        let builder = f(HashBuilder::new());
        data.insert(key.into_string(), builder.build());
        HashBuilder { data: data }
    }

    /// Return the built `Data`
    pub fn build(self) -> Data<'a> {
        Hash(self.data)
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

    pub fn push_string<T: StrAllocating>(self, value: T) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Static(value.into_string()));
        VecBuilder { data: data }
    }

    pub fn push_bool(self, value: bool) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Bool(value));
        VecBuilder { data: data }
    }

    pub fn push_vector(self, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(VecBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    pub fn push_hash(self, f: |HashBuilder<'a>| -> HashBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(HashBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    pub fn build(self) -> Data<'a> {
        Vector(self.data)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{HashBuilder, VecBuilder};
    use super::super::{Static, Bool, Vector, Hash};

    #[test]
    fn test_new_builders() {
        assert_eq!(HashBuilder::new().build(), Hash(HashMap::new()));
        assert_eq!(VecBuilder::new().build(), Vector(Vec::new()));
    }

    #[test]
    fn test_builders() {
        let mut hearthstone = HashMap::new();
        hearthstone.insert("name".to_string(), Static("Hearthstone: Heroes of Warcraft".to_string()));
        hearthstone.insert("release_date".to_string(), Static("December, 2014".to_string()));

        let mut hash = HashMap::new();
        hash.insert("first_name".to_string(), Static("Anduin".to_string()));
        hash.insert("last_name".to_string(), Static("Wrynn".to_string()));
        hash.insert("class".to_string(), Static("Priest".to_string()));
        hash.insert("died".to_string(), Bool(false));
        hash.insert("class_cards".to_string(), Vector(vec!(
            Static("Prophet Velen".to_string()),
            Hash(hearthstone))));

        assert_eq!(HashBuilder::new().insert_string("first_name", "Anduin")
            .insert_string("last_name", "Wrynn")
            .insert_string("class", "Priest")
            .insert_bool("died", false)
            .insert_vector("class_cards", |builder| {
                builder
                    .push_string("Prophet Velen")
                    .push_hash(|builder| {
                        builder
                            .insert_string("name", "Hearthstone: Heroes of Warcraft")
                            .insert_string("release_date", "December, 2014")
                    })
            })
            .build(),
            Hash(hash));
    }
}
