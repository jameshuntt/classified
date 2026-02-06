//! 
//! 
//! FEATURE NOTES
//! 
//! 
//! 
//! feature_name:std
//! deps:[std]
//! scope:[]
//! effected_lines:[]
//! corpus:true
//! 
//! 
//! 
#![cfg(feature = "std")]






use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
};

use crate::classified_data::ClassifiedData;
use zeroize::Zeroize;

/// A map structure that stores sensitive values using `ClassifiedData<V>`,
/// ensuring automatic memory zeroization upon removal or drop.
///
/// This type wraps a standard `HashMap<K, V>` but automatically classifies
/// all inserted values, protecting them with your crate's security guarantees.
///
/// # Type Parameters
/// - `K`: The key type. Must implement `Eq + Hash + Clone + Debug`.
/// - `V`: The value type. Must implement `Zeroize + Clone + Debug`.
#[derive(Debug)]
pub struct ClassifiedMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Zeroize + Clone + Debug,
{
    inner: HashMap<K, ClassifiedData<V>>,
}

impl<K, V> ClassifiedMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Zeroize + Clone + Debug,
{
    /// Creates a new, empty `ClassifiedMap`.
    ///
    /// # Example
    /// ```
    /// use classified::classified_map::ClassifiedMap;
    /// use zeroize::Zeroize;
    /// 
    /// #[derive(Zeroize, Clone, Debug)]
    /// struct SecretValue { v: u8 };
    /// 
    /// let map: ClassifiedMap<String, SecretValue> = ClassifiedMap::new();
    /// ```
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Inserts a new classified value into the map.
    ///
    /// If the key already exists, the value is replaced and the previous
    /// classified value is dropped (and zeroized).
    ///
    /// # Arguments
    /// - `key`: The map key.
    /// - `value`: The sensitive value to store.
    ///
    /// # Example
    /// ```
    /// use classified::classified_map::ClassifiedMap;
    /// use zeroize::Zeroize;
    /// 
    /// #[derive(Zeroize, Clone, Debug)]
    /// struct SecretValue { pub v: u8 };
    /// 
    /// let mut map: ClassifiedMap<String, SecretValue> = ClassifiedMap::new();
    /// 
    /// let secret_value: SecretValue = SecretValue { v: 8 };
    /// 
    /// map.insert("api_key".to_string(), secret_value);
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key, ClassifiedData::new(value));
    }

    /// Retrieves a reference to a classified value by key.
    ///
    /// # Returns
    /// - `Some(&ClassifiedData<V>)` if the key exists.
    /// - `None` otherwise.
    pub fn get(&self, key: &K) -> Option<&ClassifiedData<V>> {
        self.inner.get(key)
    }

    /// Removes the classified value for the given key.
    ///
    /// If the key exists, the associated value is dropped (and zeroized).
    pub fn remove(&mut self, key: &K) {
        self.inner.remove(key);
    }

    /// Returns an iterator over all keys in the map.
    ///
    /// The keys are returned by reference and in arbitrary order.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }

    /// Returns an iterator over all classified values in the map.
    ///
    /// Each item is a reference to a `ClassifiedData<V>`.
    pub fn values(&self) -> impl Iterator<Item = &ClassifiedData<V>> {
        self.inner.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Dummy secret type used for testing.
    #[derive(Debug, Clone, PartialEq, Eq, Zeroize)]
    struct MockSecret(String);

    #[test]
    fn inserts_and_retrieves_value() {
        let mut map = ClassifiedMap::new();
        let key = "api_key";
        let value = MockSecret("super_secret".into());

        map.insert(key, value.clone());
        let retrieved = map.get(&key).unwrap();
        assert_eq!(retrieved.expose(), &value);
    }

    #[test]
    fn removes_value() {
        let mut map = ClassifiedMap::new();
        let key = "token";
        let value = MockSecret("abc123".into());

        map.insert(key, value);
        assert!(map.get(&key).is_some());

        map.remove(&key);
        assert!(map.get(&key).is_none());
    }

    #[test]
    fn lists_all_keys() {
        let mut map = ClassifiedMap::new();
        map.insert("one", MockSecret("1".into()));
        map.insert("two", MockSecret("2".into()));

        let keys: Vec<&str> = map.keys().copied().collect();
        assert!(keys.contains(&"one"));
        assert!(keys.contains(&"two"));
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn lists_all_values() {
        let mut map = ClassifiedMap::new();
        map.insert("a", MockSecret("alpha".into()));
        map.insert("b", MockSecret("beta".into()));

        let values: Vec<_> = map.values().map(|v| v.expose().0.as_str()).collect();
        assert!(values.contains(&"alpha"));
        assert!(values.contains(&"beta"));
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn overwrite_existing_key() {
        let mut map = ClassifiedMap::new();
        let key = "id";
        map.insert(key, MockSecret("v1".into()));
        map.insert(key, MockSecret("v2".into()));

        let value = map.get(&key).unwrap();
        assert_eq!(value.expose().0, "v2");
    }
}
