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
//! //! this file could qualify for no-std as long as i make a vec and hashmap no-std drop in
//!
//! this is a thing that is a naive thing that i made
//! 
//! i will replace it with mechanisms from pkcs11
//! 
//! there will be handling of all formats, legacy and otherwise
//! 
//! 




// Phase 1: Implement Core Structs and Macros for Secure Concurrency and Cryptographic Library
use secrecy::{SecretBox};

use super::crypto_algorithm::CryptoAlgorithm;
use super::crypto_primitive::CryptoPrimitive;
use super::key_length::KeyLength;
use crate::errors::SecureError;


pub struct CryptoBuilder {
    algorithm: CryptoAlgorithm,
    key_length: Option<KeyLength>,
    zeroize: bool,
}

impl CryptoBuilder {
    pub fn new() -> Self {
        CryptoBuilder {
            algorithm: CryptoAlgorithm::RSA,
            key_length: None,
            zeroize: false,
        }
    }

    pub fn algorithm(mut self, alg: CryptoAlgorithm) -> Self {
        self.algorithm = alg;
        self
    }

    pub fn key_length(mut self, len: KeyLength) -> Self {
        self.key_length = Some(len);
        self
    }

    pub fn with_zeroize(mut self) -> Self {
        self.zeroize = true;
        self
    }

    pub fn build(self) -> Result<CryptoPrimitive, SecureError> {
        let key_length = self.key_length.ok_or_else(|| {
            SecureError::PipelineError("Key length must be specified".into())
        })?;

        // Initialize key material based on algorithm and key length
        // Placeholder key material generation
        let key_material = match self.algorithm {
            CryptoAlgorithm::AES => vec![0u8; 32], // AES-256
            CryptoAlgorithm::RSA => vec![0u8; 256], // Placeholder for RSA key
            CryptoAlgorithm::ECDSA => vec![0u8; 256], // Placeholder for ECDSA key
        };

        Ok(CryptoPrimitive {
            algorithm: self.algorithm,
            key_length,
            key_material: SecretBox::new(Box::new(key_material)),
            zeroize: self.zeroize,
        })
    }
}


#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use super::*;
    use crate::crypto::{
        crypto_algorithm::CryptoAlgorithm,
        key_length::KeyLength,
    };

    #[test]
    fn builds_with_default_rsa_settings() {
        let builder = CryptoBuilder::new().key_length(KeyLength::Bits2048);
        let result = builder.build();
        assert!(result.is_ok());

        let crypto = result.unwrap();
        assert_eq!(crypto.algorithm, CryptoAlgorithm::RSA);
        assert_eq!(crypto.key_length, KeyLength::Bits2048);
        assert_eq!(crypto.key_material.expose_secret().len(), 256);
        assert!(!crypto.zeroize);
    }

    #[test]
    fn builds_with_aes_and_zeroize() {
        let builder = CryptoBuilder::new()
            .algorithm(CryptoAlgorithm::AES)
            .key_length(KeyLength::Bits256)
            .with_zeroize();

        let result = builder.build();
        assert!(result.is_ok());

        let crypto = result.unwrap();
        assert_eq!(crypto.algorithm, CryptoAlgorithm::AES);
        assert_eq!(crypto.key_length, KeyLength::Bits256);
        assert_eq!(crypto.key_material.expose_secret().len(), 32);
        assert!(crypto.zeroize);
    }

    #[test]
    fn builds_with_ecdsa() {
        let builder = CryptoBuilder::new()
            .algorithm(CryptoAlgorithm::ECDSA)
            .key_length(KeyLength::Bits256);

        let result = builder.build();
        assert!(result.is_ok());

        let crypto = result.unwrap();
        assert_eq!(crypto.algorithm, CryptoAlgorithm::ECDSA);
        assert_eq!(crypto.key_length, KeyLength::Bits256);
        assert_eq!(crypto.key_material.expose_secret().len(), 256);
    }

    #[cfg(feature = "std")]
    #[test]
    fn fails_without_key_length() {
        let builder = CryptoBuilder::new().algorithm(CryptoAlgorithm::RSA);
        let result = builder.build();
        assert!(result.is_err());
        // assert_eq!(
        //     format!("{}", result.unwrap_err()),
        //     "PipelineError(\"Key length must be specified\")"
        // );
        assert_eq!(
            format!("{}", result.unwrap_err()), // .to_string() of your error
            "Pipeline setup failed: Key length must be specified"
        );
    }
}
