use std::collections::HashMap;
use std::convert::Into;

use Data;
use Data::{Hash, Vector};

/// `HashBuilder` is a helper type that constructs `Data` types in a HashMap
#[derive(Debug)]
pub struct HashBuilder<'a> {
    #[doc(hidden)]
    pub data: HashMap<String, Data<'a>>,
    #[doc(hidden)]
    pub partials_path: &'a str,
}

impl<'a> HashBuilder<'a> {
    /// Create a new `HashBuilder` instance
    pub fn new() -> HashBuilder<'a> {
        HashBuilder {
            data: HashMap::new(),
            partials_path: "",
        }
    }

    /// Add a `Into<Data>` to the `HashBuilder`
    ///
    /// ```rust
    /// use rustache::HashBuilder;
    /// use std::convert::Into;
    /// let data = HashBuilder::new()
    ///     .insert("game", "Hearthstone: Heroes of Warcraft");
    /// ```
    /// ```rust
    /// use rustache::{HashBuilder, VecBuilder};
    /// let data = HashBuilder::new()
    ///     .insert("classes",
    ///         VecBuilder::new()
    ///             .push("Mage".to_string())
    ///             .push("Druid".to_string())
    ///     );
    /// ```
    /// ```rust
    /// use rustache::HashBuilder;
    /// let data = HashBuilder::new()
    ///     .insert("hero1",
    ///         HashBuilder::new()
    ///             .insert("first_name", "Anduin")
    ///             .insert("last_name", "Wrynn")
    ///     )
    ///     .insert("hero2",
    ///         HashBuilder::new()
    ///             .insert("first_name", "Jaina")
    ///             .insert("last_name", "Proudmoore")
    ///     );
    /// ```
    pub fn insert<K, V>(mut self, key: K, value: V) -> HashBuilder<'a>
        where K: ToString,
              V: Into<Data<'a>>
    {
        self.data.insert(key.to_string(), value.into());
        self
    }

    /// Add a `Lambda` that accepts a String and returns a String to the `HashBuilder`
    ///
    /// ```rust
    /// use rustache::HashBuilder;
    /// let mut f = |_| { "world".to_string() };
    /// let data = HashBuilder::new()
    ///     .insert_lambda("lambda", &mut f);
    /// ```
    pub fn insert_lambda<K: ToString>(self,
                                      key: K,
                                      f: &'a mut FnMut(String) -> String)
                                      -> HashBuilder<'a> {
        self.insert(key, f)
    }

    /// Set a path to partials data
    pub fn set_partials_path(self, path: &'a str) -> HashBuilder<'a> {
        HashBuilder {
            data: self.data,
            partials_path: path,
        }
    }

    /// Return the built `Data`
    fn build(self) -> Data<'a> {
        Hash(self.data)
    }
}

impl<'a> From<HashBuilder<'a>> for Data<'a> {
    fn from(v: HashBuilder<'a>) -> Data<'a> {
        v.build()
    }
}

/// `VecBuilder` is a helper type that constructs `Data` types in a Vector
pub struct VecBuilder<'a> {
    data: Vec<Data<'a>>,
}

impl<'a> VecBuilder<'a> {
    /// Create a new `VecBuilder` instance
    pub fn new() -> VecBuilder<'a> {
        VecBuilder { data: Vec::new() }
    }

    /// Add a `Into<Data>` to the `VecBuilder`
    ///
    /// ```rust
    /// use rustache::VecBuilder;
    /// use std::convert::Into;
    /// let data = VecBuilder::new()
    ///     .push("Mage")
    ///     .push("Druid");
    /// ```
    /// ```rust
    /// use rustache::VecBuilder;
    /// let data = VecBuilder::new()
    ///     .push(VecBuilder::new()
    ///             .push("Anduin Wrynn".to_string())
    ///             .push("Jaina Proudmoore".to_string())
    ///     );
    /// ```
    /// ```rust
    /// use rustache::{HashBuilder, VecBuilder};
    /// let data = VecBuilder::new()
    ///     .push(HashBuilder::new()
    ///             .insert("first_name".to_string(), "Garrosh".to_string())
    ///             .insert("last_name".to_string(), "Hellscream".to_string())
    ///     )
    ///     .push(HashBuilder::new()
    ///             .insert("first_name".to_string(), "Malfurion".to_string())
    ///             .insert("last_name".to_string(), "Stormrage".to_string())
    ///     );
    /// ```
    pub fn push<V>(mut self, value: V) -> VecBuilder<'a>
        where V: Into<Data<'a>>
    {
        self.data.push(value.into());
        self
    }

    /// Add a `Lambda` to the `VecBuilder`
    ///
    /// ```rust
    /// use rustache::VecBuilder;
    /// let mut f = |_| { "world".to_string() };
    /// let data = VecBuilder::new()
    ///     .push_lambda(&mut f);
    /// ```
    pub fn push_lambda(self, f: &'a mut FnMut(String) -> String) -> VecBuilder<'a> {
        self.push(f)
    }

    /// Return the built `Data`
    fn build(self) -> Data<'a> {
        Vector(self.data)
    }
}

impl<'a> From<VecBuilder<'a>> for Data<'a> {
    fn from(v: VecBuilder<'a>) -> Data<'a> {
        v.build()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use {HashBuilder, VecBuilder};
    use Data::{Strng, Bool, Integer, Float, Vector, Hash, Lambda};

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
        let test_string = String::from("Conan the Sorcerian");

        let mut hearthstone = HashMap::new();
        hearthstone.insert("name".to_string(),
                           Strng("Hearthstone: Heroes of Warcraft".to_string()));
        hearthstone.insert("release_date".to_string(),
                           Strng("December, 2014".to_string()));

        let mut hash1 = HashMap::new();
        hash1.insert("first_name".to_string(), Strng("Anduin".to_string()));
        hash1.insert("last_name".to_string(), Strng("Wrynn".to_string()));
        hash1.insert("age".to_string(), Integer(21i32));
        hash1.insert("weight".to_string(), Float(120.16f64));
        hash1.insert("class".to_string(), Strng("Priest".to_string()));
        hash1.insert("died".to_string(), Bool(false));
        hash1.insert("class_cards".to_string(),
                     Vector(vec![Strng(test_string.clone()),
                                 Strng("Prophet Velen".to_string()),
                                 Hash(hearthstone)]));

        let hash2 = HashBuilder::new()
            .set_partials_path("/hearthstone")
            .insert("first_name", "Anduin")
            .insert("last_name", "Wrynn")
            .insert("age", 21i32)
            .insert("weight", 120.16f64)
            .insert("class", "Priest")
            .insert("died", false)
            .insert("class_cards",
                    VecBuilder::new()
                        .push(test_string)
                        .push("Prophet Velen")
                        .push(HashBuilder::new()
                            .insert("name", "Hearthstone: Heroes of Warcraft")
                            .insert("release_date", "December, 2014")));

        assert_eq!(Hash(hash1), Hash(hash2.data));
        assert_eq!(hash2.partials_path, "/hearthstone");
    }

    // #[test]
    // fn test_hash_lambda_builder() {
    //     // Since we can't directly compare closures, just make
    //     // sure we're threading through the builder

    //     let mut num = 10u;
    //     let data = HashBuilder::new()
    //         .insert_lambda("double", |x| {
    //             num *= 2u;
    //             x + num.to_string()
    //         })
    //         .build();

    //     match data {
    //         Hash(m) => {
    //             match *m.find_equiv(&("double")).unwrap() {
    //                 Lambda(ref f) => {
    //                     let f = &mut *f.borrow_mut();
    //                     assert_eq!((*f)("double: ".to_string()), "double: 20".to_string());
    //                     assert_eq!((*f)("double: ".to_string()), "double: 40".to_string());
    //                     assert_eq!((*f)("double: ".to_string()), "double: 80".to_string());
    //                 }
    //                 _ => panic!(),
    //             }
    //         }
    //         _ => panic!(),
    //     }
    // }

    #[test]
    fn test_vec_lambda_builder() {
        // Since we can't directly compare closures, just make
        // sure we're threading through the builder

        let mut num = 10u32;
        let mut f = |x: String| -> String {
            num *= 2u32;
            x + &num.to_string()[..]
        };
        let data = VecBuilder::new()
            .push_lambda(&mut f)
            .build();

        match data {
            Vector(m) => {
                match m[0] {
                    Lambda(ref f) => {
                        let f = &mut *f.borrow_mut();
                        assert_eq!((*f)("double: ".to_string()), "double: 20".to_string());
                        assert_eq!((*f)("double: ".to_string()), "double: 40".to_string());
                        assert_eq!((*f)("double: ".to_string()), "double: 80".to_string());
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}
