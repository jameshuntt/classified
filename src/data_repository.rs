//! Secure in-memory classified data store with concurrency support.

#![allow(unused, type_alias_bounds)]

use crate::classified_data::ClassifiedData;
use crate::errors::SecureError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use zeroize::{DefaultIsZeroes, Zeroize};

/// Alias for a secure `Mutex`-wrapped `ClassifiedData<T>`.
pub type SecureMutex<T> = Arc<Mutex<ClassifiedData<T>>>;

/// Alias for a secure `RwLock`-wrapped `ClassifiedData<T>`.
pub type SecureRwLock<T: DefaultIsZeroes> = Arc<RwLock<ClassifiedData<T>>>;

/// A thread-safe repository for securely storing and managing sensitive data.
///
/// `DataRepository<T>` ensures that all inserted values are wrapped in
/// [`ClassifiedData<T>`], enforcing secure memory handling via `Zeroize`.
///
/// It uses a `RwLock<HashMap<_, _>>` internally to support concurrent access.
pub struct DataRepository<T: DefaultIsZeroes> {
    storage: Arc<RwLock<HashMap<String, ClassifiedData<T>>>>,
}

impl<T: DefaultIsZeroes> DataRepository<T>
where
    T: Clone + Zeroize,
{
    /// Creates a new, empty `DataRepository`.
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Inserts a sensitive value under the given key.
    ///
    /// The value is wrapped in a `ClassifiedData<T>` to ensure zeroization.
    pub async fn insert(&self, key: String, data: T) -> Result<(), SecureError> {
        let sensitive_data = ClassifiedData::new(data);
        self.storage.write().await.insert(key, sensitive_data);
        Ok(())
    }

    /// Retrieves a clone of the `ClassifiedData<T>` stored at the given key.
    pub async fn get(&self, key: &str) -> Option<ClassifiedData<T>> {
        self.storage.read().await.get(key).cloned()
    }

    /// Removes the entry stored under the given key.
    ///
    /// If no entry exists, this is a no-op.
    pub async fn remove(&self, key: &str) -> Result<(), SecureError> {
        self.storage.write().await.remove(key);
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::classified_data::ClassifiedData;
    use zeroize::Zeroize;

    #[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
    struct MockData([u8; 16]);

    // Implement DefaultIsZeroes manually
    impl DefaultIsZeroes for MockData {}

    impl MockData {
        pub fn from_str(s: &str) -> Self {
            let mut bytes = [0u8; 16];
            let slice = s.as_bytes();
            let len = slice.len().min(16);
            bytes[..len].copy_from_slice(&slice[..len]);
            MockData(bytes)
        }
    }

    impl std::fmt::Display for MockData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let len = self.0.iter().position(|&b| b == 0).unwrap_or(self.0.len());
            let visible = &self.0[..len];
            write!(f, "{}", String::from_utf8_lossy(visible))
        }
    }
    
    
    #[tokio::test]
    async fn insert_and_get_returns_correct_data() {
        let repo = DataRepository::<MockData>::new();

        let key = "user:123".to_string();
        let value = MockData::from_str("secret123");

        repo.insert(key.clone(), value.clone()).await.unwrap();

        let retrieved = repo.get(&key).await.unwrap();
        assert_eq!(retrieved.expose(), &value);
    }

    #[tokio::test]
    async fn get_nonexistent_key_returns_none() {
        let repo = DataRepository::<MockData>::new();
        let result = repo.get("missing").await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn remove_key_removes_data() {
        let repo = DataRepository::<MockData>::new();

        let key = "api_token".to_string();
        let value = MockData::from_str("top_secret");

        repo.insert(key.clone(), value).await.unwrap();

        let before_remove = repo.get(&key).await;
        assert!(before_remove.is_some());

        repo.remove(&key).await.unwrap();

        let after_remove = repo.get(&key).await;
        assert!(after_remove.is_none());
    }

    #[tokio::test]
    async fn remove_nonexistent_key_is_ok() {
        let repo = DataRepository::<MockData>::new();
        let result = repo.remove("nonexistent").await;
        assert!(result.is_ok());
    }
}


