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





#[derive(Debug, Clone, PartialEq)]
pub enum CryptoAlgorithm {
    RSA,
    ECDSA,
    AES,
}
