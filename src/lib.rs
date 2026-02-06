//! # CLASSIFIED
//! Standards-based memory protection and data-centric security management.
//! 
//! This crate provides high-assurance primitives for handling sensitive 
//! cryptographic material. It implements NIST-inspired data classification 
//! strategies to prevent information leakage through physical and logical 
//! side-channels.
//!
//! ## Core Security Principles
//! * **Confidentiality:** Automated memory scrubbing via `Zeroize` on drop.
//! * **Isolation:** Opaque type wrappers to prevent accidental logging or exposure.
//! * **Resistance:** Constant-time operations via `subtle` to mitigate timing attacks.
//! * **Integrity:** Type-safe boundaries between "Public" and "Classified" data.

pub use zeroize;
pub use subtle;
pub use secrecy;

pub mod classified_data;
pub mod errors;
pub mod exposure_aware_classified_data;
pub mod classified_map;
pub mod sized_classified_data;
pub mod traits;


pub mod crypto;

pub mod config;


pub mod zeroizing_guard;

pub mod pipelines;
#[macro_use]
pub mod macros;




#[cfg(feature = "async")]
pub mod async_classified_data;
#[cfg(feature = "async")]
pub mod concurrency;
#[cfg(feature = "async")]
pub mod crypto_fallback;
#[cfg(feature = "async")]
pub mod data_repository;
#[cfg(feature = "async")]
pub mod key_manager;
#[cfg(feature = "async")]
pub mod layer_stack;
#[cfg(feature = "async")]
pub mod secure_service;
#[cfg(feature = "async")]
pub mod thread_pool_manager;