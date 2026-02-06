//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:key_manager.rs
//! description:
//! usages:none in crate yet
//! 
//! ----------------------------------------------
//! FEATURE NOTES --------------------------------
//! 
//! feature_name:async
//! deps:[tokio][async_trait]
//! scope:[impl ThreadPoolManager]
//! corpus:true
//! 
//! feature_name:std
//! deps:[std]
//! scope:[impl ThreadPoolManager]
//! corpus:false
//! 
//! ----------------------------------------------
//! CORPUS FEATURES ------------------------------
//! 
#![cfg(feature = "async")]
#![cfg(feature = "std")]


use crate::classified_data::ClassifiedData;
use crate::errors::SecureError;
use std::collections::HashMap;
use std::time::Duration;
use std::sync::Arc;

use subtle::ConstantTimeEq;
use tokio::sync::Mutex;

#[cfg(feature = "logging")]
use tracing::info;

pub type SecureMutex<T> = Arc<Mutex<T>>;

pub struct KeyManager {
    keys: SecureMutex<HashMap<String, ClassifiedData<Vec<u8>>>>,
    rotation_interval: Duration,
}

impl KeyManager {
    pub fn new(rotation_interval: Duration) -> Self {
        let keys = Arc::new(Mutex::new(HashMap::new()));
        let manager = Self {
            keys: Arc::clone(&keys),
            rotation_interval,
        };
        manager.spawn_rotation_task(keys.clone());
        manager
    }

    fn spawn_rotation_task(&self, keys: SecureMutex<HashMap<String, ClassifiedData<Vec<u8>>>>) {
        let interval = self.rotation_interval;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                let mut keys_lock = keys.lock().await;
                for (id, key) in keys_lock.iter_mut() {
                    // Replace this with actual keygen
                    *key = ClassifiedData::new(vec![0u8; 32]);

                    #[cfg(feature = "logging")]
                    info!("Key {} rotated.", id);
                }
            }
        });
    }

    pub async fn accept_key(&self, id: &str, length: usize, key: Vec<u8>) -> Result<(), SecureError> {
        if length == 0 {
            return Err(SecureError::InvalidKeyLength);
        }
        
        if key.len() != length {
            return Err(SecureError::InvalidKeyLength);
        }

        let sensitive = ClassifiedData::new(key);

        self.keys.lock().await.insert(id.to_string(), sensitive);
        Ok(())
    }

    pub async fn get_key(&self, id: &str) -> Option<ClassifiedData<Vec<u8>>> {
        let keys = self.keys.lock().await;
        keys.get(id).cloned()
    }

    pub async fn remove_key(&self, id: &str) -> Result<(), SecureError> {
        let mut keys = self.keys.lock().await;
        keys.remove(id);
        Ok(())
    }

    pub async fn compare_key(&self, id: &str, other: &[u8]) -> Option<bool> {
        let keys = self.keys.lock().await;
        keys.get(id).map(|stored| {
            stored.expose().ct_eq(other).into()
        })
    }

    pub fn with_rotation(rotation_interval: Duration) -> Self {
        let keys = Arc::new(Mutex::new(HashMap::new()));
        let manager = Self {
            keys: Arc::clone(&keys),
            rotation_interval,
        };
        manager.spawn_rotation_task(keys.clone());
        manager
    }

    pub fn without_rotation() -> Self {
        let keys = Arc::new(Mutex::new(HashMap::new()));
        Self {
            keys,
            rotation_interval: Duration::from_secs(0),
        }
    }
}
