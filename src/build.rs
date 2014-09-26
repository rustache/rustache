pub use std::collections::HashMap;

use super::{Data, Strng, Bool, Vector, Hash};

/// `HashBuilder` is a helper type that constructs `Data` types in a HashMap
#[deriving(Show)]
pub struct HashBuilder<'a> {
    pub data: HashMap<String, Data<'a>>,
    pub partials_path: &'a str
}

impl<'a> HashBuilder<'a> {
    /// Create a new `HashBuilder` instance
    #[inline]
    pub fn new() -> HashBuilder<'a> {
        HashBuilder {
            data: HashMap::new(),
            partials_path: ""
        }
    }

    /// Add a `String` to the `HashBuilder`
    ///
    /// ```rust
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_string("game", "Hearthstone: Heroes of Warcraft")
    ///     .build();
    /// ```
    #[inline]
    pub fn insert_string<K: StrAllocating, V: StrAllocating>(self, key: K, value: V) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        data.insert(key.into_string(), Strng(value.into_string()));
        HashBuilder { data: data, partials_path: self.partials_path }
    }

    /// Add a `Boolean` to the `HashBuilder`
    ///
    /// ```rust
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_bool("playing", true)
    ///     .build();
    /// ```
    #[inline]
    pub fn insert_bool<K: StrAllocating>(self, key: K, value: bool) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        data.insert(key.into_string(), Bool(value));
        HashBuilder { data: data, partials_path: self.partials_path }
    }

    /// Add a `Vector` to the `HashBuilder`
    ///
    /// ```rust
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_vector("classes", |builder| {
    ///         builder
    ///             .push_string("Mage".to_string())
    ///             .push_string("Druid".to_string())
    ///     })
    ///     .build();   
    /// ```
    #[inline]
    pub fn insert_vector<K: StrAllocating>(self, key: K, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        let builder = f(VecBuilder::new());
        data.insert(key.into_string(), builder.build());
        HashBuilder { data: data, partials_path: self.partials_path }
    }  

    /// Add a `Hash` to the `HashBuilder`
    /// 
    /// ```rust
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_hash("hero1", |builder| {
    ///         builder
    ///             .insert_string("first_name", "Anduin")
    ///             .insert_string("last_name", "Wrynn")
    ///     })
    ///     .insert_hash("hero2", |builder| {
    ///         builder
    ///             .insert_string("first_name", "Jaina")
    ///             .insert_string("last_name", "Proudmoore")    
    ///     })
    ///     .build();
    /// ```
    #[inline]
    pub fn insert_hash<K: StrAllocating>(self, key: K, f: |HashBuilder<'a>| -> HashBuilder<'a>) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        let builder = f(HashBuilder::new());
        data.insert(key.into_string(), builder.build());
        HashBuilder { data: data, partials_path: self.partials_path }
    }

    /// Set a path to partials data
    #[inline]
    pub fn set_partials_path(self, path: &'a str) -> HashBuilder<'a> {
        let HashBuilder { data, mut partials_path } = self;
        partials_path = path;
        HashBuilder { data: data, partials_path: partials_path }
    }

    /// Return the built `Data`
    #[inline]
    pub fn build(self) -> Data<'a> {
        Hash(self.data)
    }
}

/// `VecBuilder` is a helper type that constructs `Data` types in a Vector
pub struct VecBuilder<'a> {
    data: Vec<Data<'a>>
}

impl<'a> VecBuilder<'a> {
    /// Create a new `VecBuilder` instance
    #[inline]
    pub fn new() -> VecBuilder<'a> {
        VecBuilder {
            data: Vec::new()
        }
    }

    /// Add a `String` to the `VecBuilder`
    ///
    /// ```rust
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_string("Mage")
    ///     .push_string("Druid")
    ///     .build();
    /// ```
    #[inline]
    pub fn push_string<T: StrAllocating>(self, value: T) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Strng(value.into_string()));
        VecBuilder { data: data }
    }

    /// Add a `Boolean` to the `VecBuilder`
    ///
    /// ```rust
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_bool(true)
    ///     .push_bool(false)
    ///     .build();
    /// ```
    #[inline]
    pub fn push_bool(self, value: bool) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Bool(value));
        VecBuilder { data: data }
    }

    /// Add a `Vector` to the `VecBuilder`
    ///
    /// ```rust
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_vector(|builder| {
    ///         builder
    ///             .push_string("Anduin Wrynn".to_string())
    ///             .push_string("Jaina Proudmoore".to_string())
    ///     })
    ///     .build();
    /// ```
    #[inline]
    pub fn push_vector(self, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(VecBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    /// Add a `Hash` to the `VecBuilder`
    ///
    /// ```rust
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_hash(|builder| {
    ///         builder
    ///             .insert_string("first_name".to_string(), "Garrosh".to_string())
    ///             .insert_string("last_name".to_string(), "Hellscream".to_string())       
    ///     })
    ///     .push_hash(|builder| {
    ///         builder
    ///             .insert_string("first_name".to_string(), "Malfurion".to_string())
    ///             .insert_string("last_name".to_string(), "Stormrage".to_string())    
    ///     })
    ///     .build();
    /// ```
    #[inline]
    pub fn push_hash(self, f: |HashBuilder<'a>| -> HashBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(HashBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    /// Return the built `Data`
    #[inline]
    pub fn build(self) -> Data<'a> {
        Vector(self.data)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{HashBuilder, VecBuilder};
    use super::super::{Strng, Bool, Vector, Hash};

    #[test]
    fn test_new_builders() {
        assert_eq!(HashBuilder::new().build(), Hash(HashMap::new()));
        assert_eq!(VecBuilder::new().build(), Vector(Vec::new()));
    }

    #[test]
    fn test_set_partials_path() {
        let hash = HashBuilder::new().set_partials_path("/path");
        assert_eq!(hash.partials_path, "/path");
    }

    #[test]
    fn test_builders() {
        let mut hearthstone = HashMap::new();
        hearthstone.insert("name".to_string(), Strng("Hearthstone: Heroes of Warcraft".to_string()));
        hearthstone.insert("release_date".to_string(), Strng("December, 2014".to_string()));

        let mut hash1 = HashMap::new();
        hash1.insert("first_name".to_string(), Strng("Anduin".to_string()));
        hash1.insert("last_name".to_string(), Strng("Wrynn".to_string()));
        hash1.insert("class".to_string(), Strng("Priest".to_string()));
        hash1.insert("died".to_string(), Bool(false));
        hash1.insert("class_cards".to_string(), Vector(vec!(
            Strng("Prophet Velen".to_string()),
            Hash(hearthstone))));

        let mut hash2 = HashBuilder::new().set_partials_path("/hearthstone")
                            .insert_string("first_name", "Anduin")
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
                            });

        assert_eq!(Hash(hash1), Hash(hash2.data));
        assert_eq!(hash2.partials_path, "/hearthstone");
    }
}
