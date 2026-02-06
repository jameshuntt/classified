//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:exposure_aware_classified_data.rs
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
#![cfg(feature = "std")]



use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH}
};

use secrecy::{ExposeSecret, SecretBox};
use zeroize::Zeroize;

#[cfg(feature = "logging")]
use tracing::warn;

#[derive(Debug)]
pub enum ExposurePurpose {
    Signing,
    Decryption,
    KeyWrapping,
    Audit,
}

pub struct ExposureAwareClassifiedData<T: Zeroize> {
    data: SecretBox<T>,
}

static EXPOSURE_COUNT: AtomicUsize = AtomicUsize::new(0);

impl_new!(ExposureAwareClassifiedData);
impl<T: Zeroize> ExposureAwareClassifiedData<T> {
    #[must_use = "You must never ignore sensitive data"]
    /// Safely expose secret with logging + audit hooks
    pub fn expose(&self) -> &T {
        let count = EXPOSURE_COUNT.fetch_add(1, Ordering::SeqCst);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        #[cfg(feature = "logging")]
        // 👁️ Hooked exposure log
        warn!(target: "sensitive", time = %now, count, "Sensitive value exposed");

        self.data.expose_secret()
    }

    pub fn exposure_count() -> usize {
        EXPOSURE_COUNT.load(Ordering::SeqCst)
    }

    pub fn expose_for(&self, purpose: ExposurePurpose) -> &T {
        #[cfg(feature = "logging")]
        warn!(target: "security_audit", purpose = ?purpose, "Sensitive material accessed");
        self.data.expose_secret()
    }
}
impl_for_generics_no_trait!(
    ExposureAwareClassifiedData<T>,
    pub fn exposed(&self) -> &T {
        let count = EXPOSURE_COUNT.fetch_add(1, Ordering::SeqCst);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        #[cfg(feature = "logging")]
        // 👁️ Hooked exposure log
        warn!(target: "sensitive", time = %now, count, "Sensitive value exposed");

        self.data.expose_secret()
    }
);

impl ExposureAwareClassifiedData<Vec<u8>> {
    pub async fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut Vec<u8>),
    {
        let mut data = self.data.expose_secret().clone();
        f(&mut data);
    }
}

use crate::{impl_clone, impl_ct, impl_debug, impl_for_generics_no_trait, impl_generic_drop, impl_new};
impl_clone!(ExposureAwareClassifiedData);
impl_generic_drop!(ExposureAwareClassifiedData<T>, data);
// impl_drop!(ExposureAwareClassifiedData);
impl_debug!(ExposureAwareClassifiedData);
impl_ct!(ExposureAwareClassifiedData);







#[cfg(test)]
mod tests {
    use super::*;
    use subtle::{ConstantTimeEq};
    
    #[cfg(feature = "concurrency")]
    use tracing_subscriber::{fmt, EnvFilter};

    #[cfg(feature = "concurrency")]
    fn init_tracing() {
        let _ = tracing::subscriber::set_default(
            fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .finish(),
        );
    }

    #[cfg(feature = "concurrency")]
    #[test]
    fn exposes_value_and_increments_count() {
        init_tracing();
        let sensitive = ExposureAwareClassifiedData::new(1234u32);
        let before = ExposureAwareClassifiedData::<u32>::exposure_count();
        assert_eq!(*sensitive.expose(), 1234);
        let after = ExposureAwareClassifiedData::<u32>::exposure_count();
        assert_eq!(after, before + 1);
    }

    #[test]
    fn clone_preserves_value_but_not_count() {
        let sensitive = ExposureAwareClassifiedData::new("secure".to_string());
        let cloned = sensitive.clone();
        assert_eq!(sensitive.expose(), cloned.expose());
    }

    #[test]
    fn debug_is_redacted() {
        let data = ExposureAwareClassifiedData::new(vec![1, 2, 3]);
        let dbg = format!("{:?}", data);
        assert_eq!(dbg, "ExposureAwareClassifiedData(<REDACTED>)");
    }

    #[tokio::test]
    async fn ct_eq_works_as_expected() {
        let a = ExposureAwareClassifiedData::new(vec![1, 2, 3]);
        let b = ExposureAwareClassifiedData::new(vec![1, 2, 3]);
        let c = ExposureAwareClassifiedData::new(vec![4, 5, 6]);

        assert!(bool::from(a.ct_eq(&b)));
        assert!(!bool::from(a.ct_eq(&c)));
    }

    #[tokio::test]
    async fn async_update_clones_and_modifies() {
        let data = ExposureAwareClassifiedData::new(vec![10, 20, 30]);
        data.update(|v| {
            v[0] = 42;
        }).await;

        // update does not mutate in-place, it clones and modifies the clone.
        // So original stays untouched
        assert_eq!(data.expose(), &[10, 20, 30]);
    }
}
