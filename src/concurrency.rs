//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:datd_repository.rs
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

pub mod actor_channel;
pub mod actors;
pub mod csp;
pub mod futures;
pub mod streams;