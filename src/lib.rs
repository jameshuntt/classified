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