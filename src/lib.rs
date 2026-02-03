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

pub mod async_classified_data;
pub mod classified_data;
pub mod data_repository;
pub mod errors;
pub mod exposure_aware_classified_data;
pub mod classified_map;
pub mod sized_classified_data;
pub mod traits;

#[macro_use]
pub mod macros;