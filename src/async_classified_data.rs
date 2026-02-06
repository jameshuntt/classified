//! 
//! 
//! FEATURE NOTES
//! 
//! 
//! 
//! feature_name:async
//! deps:[tokio][async_trait]
//! scope:[]
//! effected_lines:[]
//! corpus:true
//! 
//! feature_name:std
//! deps:[std]
//! scope:[]
//! effected_lines:[]
//! corpus:true
//! 
//! 
//! 
#![cfg(feature = "async")]




//! Async-sensitive secure wrapper for confidential data.
//!
//! This crate provides a `AsyncClassifiedData<T>` type that combines:
//! - [`SecretBox`] from `secrecy` for memory-safe secret handling
//! - `tokio::Mutex` for async-safe access
//! - `Zeroize` and `subtle` for secure memory wiping and constant-time comparisons
//!
//! Ideal for storing passwords, tokens, keys, or other sensitive in-memory data
//! in async contexts like servers, agents, or orchestrators.
//!
//! # Features
//! - Cloneable with shallow `Arc` semantics
//! - Zeroizes memory when dropped
//! - Redacted `Debug` output
//! - Optional constant-time comparison (`ct_eq`)
//!
//! ## Example
//! ```rust
//! # use classified::async_classified_data::AsyncClassifiedData;
//! # use tokio;
//! # #[tokio::main]
//! # async fn main() {
//! let secret = AsyncClassifiedData::new("top_secret".to_string());
//! assert_eq!(secret.expose().await, "top_secret");
//!
//! secret.update(|v| *v = "updated".to_string()).await;
//! assert_eq!(secret.expose().await, "updated");
//! # }
//! ```

use std::{fmt, sync::Arc};
use secrecy::{ExposeSecret, ExposeSecretMut, SecretBox};
use zeroize::Zeroize;
use tokio::sync::Mutex;

/// A wrapper for managing sensitive data in async environments with secure memory handling.
///
/// This type encapsulates a [`SecretBox<T>`] inside a `tokio::Mutex`, allowing concurrent
/// async-safe access while preserving confidentiality guarantees.
///
/// # Security Guarantees
/// - Underlying data is allocated on the heap and zeroized on drop
/// - Memory-safe interior mutability via [`Mutex`]
/// - Constant-time equality support (via [`subtle::ConstantTimeEq`])
/// - Redacted debug output
#[derive(Clone)]
pub struct AsyncClassifiedData<T: Zeroize> {
    inner: Arc<Mutex<SecretBox<T>>>,
}

impl<T: Zeroize> AsyncClassifiedData<T> {
    /// Creates a new instance wrapping the provided sensitive data.
    ///
    /// # Arguments
    /// * `data` - The sensitive value to protect (must implement `Zeroize`)
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SecretBox::new(Box::new(data)))),
        }
    }

    /// Asynchronously exposes a **cloned** copy of the sensitive value.
    ///
    /// This avoids leaking the original reference and ensures the secure box remains intact.
    ///
    /// # Panics
    /// Panics if `T` does not implement `Clone`.
    pub async fn expose(&self) -> T
    where
        T: Clone,
    {
        let locked = self.inner.lock().await;
        locked.expose_secret().clone()
    }

    /// Asynchronously applies a function to **mutate** the sensitive value in place.
    ///
    /// Use this to modify the inner value securely without exposing it.
    ///
    /// # Arguments
    /// * `f` - A closure that receives a mutable reference to the underlying `T`
    pub async fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut locked = self.inner.lock().await;
        f(locked.expose_secret_mut());
    }

    /// Asynchronously exposes a **mutable reference** to the sensitive value for scoped use.
    ///
    /// Similar to `update`, but better naming when the operation is purely "read-write" rather
    /// than a transformation.
    pub async fn expose_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut locked = self.inner.lock().await;
        f(locked.expose_secret_mut());
    }
}

impl<T: Zeroize> fmt::Debug for AsyncClassifiedData<T> {
    /// Prevents accidental logging of sensitive data.
    ///
    /// Always outputs: `AsyncClassifiedData(REDACTED)`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AsyncClassifiedData(REDACTED)")
    }
}

use subtle::{Choice, ConstantTimeEq};

impl<T: ConstantTimeEq + Zeroize> ConstantTimeEq for AsyncClassifiedData<T> {
    /// Performs constant-time equality comparison between two sensitive values.
    ///
    /// > **Warning**: This uses a blocking executor (`block_on`) and is not recommended in
    /// production async contexts where deadlocks may occur. Use with caution.
    fn ct_eq(&self, other: &Self) -> Choice {
        futures::executor::block_on(async {
            let a = self.inner.lock().await;
            let b = other.inner.lock().await;
            a.expose_secret().ct_eq(b.expose_secret())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_expose_returns_clone() {
        let s = AsyncClassifiedData::new("secret".to_string());
        assert_eq!(s.expose().await, "secret");
    }

    #[tokio::test]
    async fn test_update_changes_value() {
        let s = AsyncClassifiedData::new("abc".to_string());
        s.update(|v| *v = "xyz".to_string()).await;
        assert_eq!(s.expose().await, "xyz");
    }

    #[tokio::test]
    async fn test_debug_is_redacted() {
        let s = AsyncClassifiedData::new("secret".to_string());
        let debug_output = format!("{:?}", s);
        assert_eq!(debug_output, "AsyncClassifiedData(REDACTED)");
    }
}
