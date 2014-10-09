use std::cell::RefCell;
use std::collections::HashMap;

use {Data, Strng, Bool, Integer, Float, Vector, Hash, Lambda};

/// `HashBuilder` is a helper type that constructs `Data` types in a HashMap
#[deriving(Show)]
pub struct HashBuilder<'a> {
    #[doc(hidden)]
    pub data: HashMap<String, Data<'a>>,
    #[doc(hidden)]
    pub partials_path: &'a str
}

impl<'a> HashBuilder<'a> {
    /// Create a new `HashBuilder` instance
    pub fn new() -> HashBuilder<'a> {
        HashBuilder {
            data: HashMap::new(),
            partials_path: ""
        }
    }

    /// Add a `String` to the `HashBuilder`
    ///
    /// ```ignore
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_string("game", "Hearthstone: Heroes of Warcraft");
    /// ```
    pub fn insert_string<K: StrAllocating, V: StrAllocating>(self, key: K, value: V) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        data.insert(key.into_string(), Strng(value.into_string()));
        HashBuilder { data: data, partials_path: partials_path }
    }

    /// Add a `Boolean` to the `HashBuilder`
    ///
    /// ```ignore
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_bool("playing", true);
    /// ```
    pub fn insert_bool<K: StrAllocating>(self, key: K, value: bool) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        data.insert(key.into_string(), Bool(value));
        HashBuilder { data: data, partials_path: partials_path }
    }

    /// Add an `Integer` to the `HashBuilder`
    ///
    /// ```ignore
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_int("age", 10i)
    ///     .insert_int("drinking age", -21i);
    /// ```
    pub fn insert_int<K: StrAllocating>(self, key: K, value: int) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        data.insert(key.into_string(), Integer(value));
        HashBuilder { data: data, partials_path: partials_path }
    }

    /// Add a `Float` to the `HashBuilder`
    ///
    /// ```ignore
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_float("pi", 3.141596f64)
    ///     .insert_float("phi", 1.61803398875f64);
    /// ```
    pub fn insert_float<K: StrAllocating>(self, key: K, value: f64) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        data.insert(key.into_string(), Float(value));
        HashBuilder { data: data, partials_path: partials_path }
    }

    /// Add a `Vector` to the `HashBuilder`
    ///
    /// ```ignore
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_vector("classes", |builder| {
    ///         builder
    ///             .push_string("Mage".to_string())
    ///             .push_string("Druid".to_string())
    ///     });  
    /// ```
    pub fn insert_vector<K: StrAllocating>(self, key: K, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        let builder = f(VecBuilder::new());
        data.insert(key.into_string(), builder.build());
        HashBuilder { data: data, partials_path: partials_path }
    }  

    /// Add a `Hash` to the `HashBuilder`
    /// 
    /// ```ignore
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
    ///     });
    /// ```
    pub fn insert_hash<K: StrAllocating>(self, key: K, f: |HashBuilder<'a>| -> HashBuilder<'a>) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        let builder = f(HashBuilder::new());
        data.insert(key.into_string(), builder.build());
        HashBuilder { data: data, partials_path: partials_path }
    }

    /// Add a `Lambda` that accepts a String and returns a String to the `HashBuilder`
    ///
    /// ```ignore
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert_lambda("lambda", |_| {
    ///         "world".to_string()               
    ///     });
    /// ```
    pub fn insert_lambda<K: StrAllocating>(self, key: K, f: |String|: 'a -> String) -> HashBuilder<'a> {
        let HashBuilder { mut data, partials_path } = self;
        data.insert(key.into_string(), Lambda(RefCell::new(f)));
        HashBuilder { data: data, partials_path: partials_path }
    }

    /// Set a path to partials data
    pub fn set_partials_path(self, path: &'a str) -> HashBuilder<'a> {
        HashBuilder { data: self.data, partials_path: path }
    }

    /// Return the built `Data`
    fn build(self) -> Data<'a> {
        Hash(self.data)
    }
}

/// `VecBuilder` is a helper type that constructs `Data` types in a Vector
pub struct VecBuilder<'a> {
    data: Vec<Data<'a>>
}

impl<'a> VecBuilder<'a> {
    /// Create a new `VecBuilder` instance
    pub fn new() -> VecBuilder<'a> {
        VecBuilder {
            data: Vec::new()
        }
    }

    /// Add a `String` to the `VecBuilder`
    ///
    /// ```ignore
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_string("Mage")
    ///     .push_string("Druid");
    /// ```
    pub fn push_string<T: StrAllocating>(self, value: T) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Strng(value.into_string()));
        VecBuilder { data: data }
    }

    /// Add a `Bool` to the `VecBuilder`
    ///
    /// ```ignore
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_bool(true)
    ///     .push_bool(false);
    /// ```
    pub fn push_bool(self, value: bool) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Bool(value));
        VecBuilder { data: data }
    }

    /// Add an `Integer` to the `VecBuilder`
    ///
    /// ```ignore
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_int(10i)
    ///     .push_int(-21i);
    /// ```
    pub fn push_int(self, value: int) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Integer(value));
        VecBuilder { data: data }
    }

    /// Add a `Float` to the `VecBuilder`
    ///
    /// ```ignore
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_float(10.356356f64)
    ///     .push_float(-21.34956230456f64);
    /// ```
    pub fn push_float(self, value: f64) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        data.push(Float(value));
        VecBuilder { data: data }
    }

    /// Add a `Vector` to the `VecBuilder`
    ///
    /// ```ignore
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_vector(|builder| {
    ///         builder
    ///             .push_string("Anduin Wrynn".to_string())
    ///             .push_string("Jaina Proudmoore".to_string())
    ///     });
    /// ```
    pub fn push_vector(self, f: |VecBuilder<'a>| -> VecBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(VecBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    /// Add a `Hash` to the `VecBuilder`
    ///
    /// ```ignore
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
    ///     });
    /// ```
    pub fn push_hash(self, f: |HashBuilder<'a>| -> HashBuilder<'a>) -> VecBuilder<'a> {
        let VecBuilder { mut data } = self;
        let builder = f(HashBuilder::new());
        data.push(builder.build());
        VecBuilder { data: data }
    }

    /// Add a `Lambda` to the `VecBuilder`
    ///
    /// ```ignore
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push_lambda(|lambda| {
    ///         "world".to_string()
    ///     });
    /// ```
    pub fn push_lambda(self, f: |String|: 'a -> String) -> VecBuilder <'a> {
        let VecBuilder { mut data } = self;
        data.push(Lambda(RefCell::new(f)));
        VecBuilder { data: data }
    }

    /// Return the built `Data`
    fn build(self) -> Data<'a> {
        Vector(self.data)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{HashBuilder, VecBuilder};
    use super::super::{Strng, Bool, Integer, Float, Vector, Hash, Lambda};

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
        hash1.insert("age".to_string(), Integer(21i));
        hash1.insert("weight".to_string(), Float(120.16f64));
        hash1.insert("class".to_string(), Strng("Priest".to_string()));
        hash1.insert("died".to_string(), Bool(false));
        hash1.insert("class_cards".to_string(), Vector(vec!(
            Strng("Prophet Velen".to_string()),
            Hash(hearthstone))));

        let hash2 = HashBuilder::new().set_partials_path("/hearthstone")
                        .insert_string("first_name", "Anduin")
                        .insert_string("last_name", "Wrynn")
                        .insert_int("age", 21i)
                        .insert_float("weight", 120.16f64)
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

    #[test]
    fn test_hash_lambda_builder() {
        // Since we can't directly compare closures, just make
        // sure we're threading through the builder

        let mut num = 10u;
        let data = HashBuilder::new()
            .insert_lambda("double", |x| {
                num *= 2u;
                x + num.to_string()
            })
            .build();

        match data {
            Hash(m) => {
                match *m.find_equiv(&("double")).unwrap() {
                    Lambda(ref f) => {
                        let f = &mut *f.borrow_mut();
                        assert_eq!((*f)("double: ".to_string()), "double: 20".to_string());
                        assert_eq!((*f)("double: ".to_string()), "double: 40".to_string());
                        assert_eq!((*f)("double: ".to_string()), "double: 80".to_string());
                    }
                    _ => fail!(),
                }
            }
            _ => fail!(),
        }
    }

    #[test]
    fn test_vec_lambda_builder() {
        // Since we can't directly compare closures, just make
        // sure we're threading through the builder

        let mut num = 10u;
        let data = VecBuilder::new()
            .push_lambda(|x| {
                num *= 2u;
                x + num.to_string()
            })
            .build();

        match data {
            Vector(m) => {
                match m.as_slice() {
                    [Lambda(ref f)] => {
                        let f = &mut *f.borrow_mut();
                        assert_eq!((*f)("double: ".to_string()), "double: 20".to_string());
                        assert_eq!((*f)("double: ".to_string()), "double: 40".to_string());
                        assert_eq!((*f)("double: ".to_string()), "double: 80".to_string());
                    }
                    _ => fail!(),
                }
            }
            _ => fail!(),
        }
    }
}
