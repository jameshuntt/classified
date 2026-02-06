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
//! feature_name:logging
//! deps:[tracing]
//! scope:[]
//! effected_lines:[]
//! corpus:false
//! 
//! feature_name:std
//! deps:[std]
//! scope:[]
//! effected_lines:[]
//! corpus:false
//! 
//! 
//! 
//! 
#![cfg(feature = "async")]
//! 
//! 
//! 
//! 
//! 
//! filename:
//! 
//! 
//! usages:
//! 
//! 
//! 
//! 
#![cfg_attr(feature = "no-clone", doc = "⚠️ Cloning is disabled unless `no-clone` is unset.")]

use secrecy::{ExposeSecret, SecretBox};
use zeroize::Zeroize;

/// A secure wrapper for sensitive data that ensures memory is zeroed on drop,
/// and access is tightly controlled through secure methods.
///
/// `ClassifiedData<T>` is designed for managing secret values (keys, tokens, etc.)
/// that implement [`Zeroize`]. It leverages [`SecretBox`] from the `secrecy` crate
/// for automatic zeroing, and optionally restricts cloning via a feature flag.
pub struct ClassifiedData<T: Zeroize> {
    data: SecretBox<T>,
}

impl<T: Zeroize> ClassifiedData<T> {
    /// Create a new classified value, wrapping the given data securely.
    ///
    /// The inner data will be zeroed on drop, and access is controlled.
    ///
    /// # Example
    /// ```
    /// use classified::classified_data::ClassifiedData;
    /// let secret = ClassifiedData::new("my-api-key".to_string());
    /// ```
    pub fn new(data: T) -> Self {
        ClassifiedData {
            data: SecretBox::new(Box::new(data)),
        }
    }

    // /// Expose a reference to the inner value.
    // ///
    // /// ⚠️ Use with care. This is a read-only view of sensitive data.
    // #[must_use = "You must never ignore confidential data"]
    // pub fn expose(&self) -> &T {
    //     self.data.expose_secret()
    // }
}

impl_expose!(ClassifiedData);

impl<T: Clone + Zeroize> ClassifiedData<T> {
    /// Clone the inner data, apply a mutation, and discard the result.
    ///
    /// This avoids mutating the original data and can be used for
    /// computations like hashing or transformations in a controlled way.
    pub async fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut data = self.data.expose_secret().clone();
        f(&mut data);
    }
}

impl<T: AsRef<[u8]> + Zeroize> ClassifiedData<T> {
    /// Check if the inner byte-like data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.expose_secret().as_ref().is_empty()
    }
}

use subtle::{ConstantTimeEq, Choice};

/// Implements constant-time equality for `ClassifiedData<Vec<u8>>`,
/// suitable for comparing secret values without leaking timing info.
impl ConstantTimeEq for ClassifiedData<Vec<u8>> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.expose().ct_eq(other.expose())
    }
}

use crate::impl_expose;
use crate::traits::ClassifiedEq;

/// Provides a semantic wrapper over `ct_eq`, returning a boolean.
impl ClassifiedEq for ClassifiedData<Vec<u8>> {
    fn classified_eq(&self, rhs: &Self) -> bool {
        self.ct_eq(rhs).into()
    }
}

/// Ensures that the inner secret is zeroized before memory is freed.
impl<T: Zeroize> Drop for ClassifiedData<T> {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}

// if for some reason i decide to 
// use macros for impls
// use crate::impl_drop;
// impl_drop!(ClassifiedData);

#[cfg(not(feature = "no-clone"))]
/// Allows cloning of classified data only if the feature `no-clone` is not set.
///
/// ⚠️ Cloning secrets can be dangerous and should only be enabled when necessary.
impl<T: Clone + Zeroize> Clone for ClassifiedData<T> {
    fn clone(&self) -> Self {
        ClassifiedData {
            data: SecretBox::new(Box::new(self.data.expose_secret().clone())),
        }
    }
}

#[cfg(feature = "no-clone")]
/// Prevents cloning of classified data when `no-clone` is enabled,
/// enforcing strong immutability of secrets.
impl<T: Clone + Zeroize> Clone for ClassifiedData<T> {
    fn clone(&self) -> Self {
        panic!("Cloning sensitive data is forbidden");
    }
}

use std::ops::Deref;

/// Enables dereferencing to the inner value for ergonomic use.
///
/// ⚠️ This should be used carefully, especially with traits that
/// could expose the inner secret (e.g., `Debug`, `Display`).
impl<T: Zeroize> Deref for ClassifiedData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.expose_secret()
    }
}

use std::fmt::{Debug, Formatter, Result};

/// Hides inner secrets from accidental logging or printing.
///
/// Always shows `<redacted>`, regardless of the wrapped value.
impl<T: Zeroize> Debug for ClassifiedData<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ClassifiedData(<redacted>)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use subtle::ConstantTimeEq;

    /// Tests that data can be wrapped and safely exposed.
    #[test]
    fn creates_and_exposes_data() {
        let sensitive = ClassifiedData::new(vec![1, 2, 3]);
        assert_eq!(sensitive.expose(), &[1, 2, 3]);
    }

    /// Verifies that `.update()` clones and modifies a temporary copy,
    /// and the original value remains unchanged.
    #[tokio::test]
    async fn update_clones_and_applies_function_but_does_not_mutate_original() {
        let sensitive = ClassifiedData::new(vec![1, 2, 3]);

        sensitive.update(|v| {
            v.push(9);
        }).await;

        assert_eq!(sensitive.expose(), &[1, 2, 3]);
    }

    /// Tests equality between two equal secrets using constant-time comparison.
    #[test]
    fn constant_time_eq_works_correctly() {
        let a = ClassifiedData::new(vec![42; 8]);
        let b = ClassifiedData::new(vec![42; 8]);
        let c = ClassifiedData::new(vec![99; 8]);

        assert!(a.ct_eq(&b).unwrap_u8() == 1);
        assert!(a.ct_eq(&c).unwrap_u8() == 0);
    }

    /// Tests equality behavior when secrets have different lengths.
    #[test]
    fn handles_different_lengths_in_ct_eq() {
        let short = ClassifiedData::new(vec![1, 2, 3]);
        let long = ClassifiedData::new(vec![1, 2, 3, 4]);
        assert_eq!(short.ct_eq(&long).unwrap_u8(), 0);
    }

    /// Verifies that dropping a `ClassifiedData` value doesn’t panic
    /// and behaves as expected under async runtime.
    #[tokio::test]
    async fn sensitive_data_clears_on_drop() {
        let data = ClassifiedData::new(vec![1u8, 2, 3]);
        assert_eq!(data.expose(), &[1, 2, 3]);

        drop(data); // triggers Drop, calls zeroize
    }

    /// Tests whether the `Drop` implementation is called, using a flag.
    #[test]
    fn zeroize_on_drop() {
        use std::cell::RefCell;
        thread_local! {
            static DROP_FLAG: RefCell<bool> = RefCell::new(false);
        }

        struct Tracked(Vec<u8>);

        impl Drop for Tracked {
            fn drop(&mut self) {
                DROP_FLAG.with(|f| *f.borrow_mut() = true);
            }
        }

        impl Zeroize for Tracked {
            fn zeroize(&mut self) {
                for b in &mut self.0 {
                    *b = 0;
                }
            }
        }

        {
            let _ = ClassifiedData::new(Tracked(vec![1, 2, 3]));
            // Drop happens here at end of scope
        }

        assert!(DROP_FLAG.with(|f| *f.borrow()), "Drop was not called");
    }
}
