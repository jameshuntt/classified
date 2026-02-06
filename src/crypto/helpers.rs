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

use crate::{
    crypto::{
        key_length::KeyLength,
        crypto_algorithm::CryptoAlgorithm
    },
    errors::SecureError
};


// Helper functions to parse algorithms and key lengths from config
pub fn parse_algorithm(
    alg: &Option<String>
) -> Result<CryptoAlgorithm, SecureError> {
    match alg.as_deref() {
        Some("AES-256") => Ok(CryptoAlgorithm::AES),
        Some("RSA") => Ok(CryptoAlgorithm::RSA),
        Some("ECDSA") => Ok(CryptoAlgorithm::ECDSA),
        _ => Err(SecureError::PipelineError(
            "Unsupported algorithm".into()
        )),
    }
}

pub fn parse_key_length(
    alg: &Option<String>
) -> Result<KeyLength, SecureError> {
    match alg.as_deref() {
        Some("AES-256") => Ok(KeyLength::Bits256),
        Some("RSA") => Ok(KeyLength::Bits2048),
        Some("ECDSA") => Ok(KeyLength::Bits256),
        _ => Err(SecureError::PipelineError(
            "Unsupported algorithm for key length".into()
        )),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{crypto_algorithm::CryptoAlgorithm, key_length::KeyLength};
    use crate::errors::SecureError;

    #[test]
    fn parses_valid_algorithm_strings() {
        assert_eq!(
            parse_algorithm(&Some("AES-256".into())).unwrap(),
            CryptoAlgorithm::AES
        );
        assert_eq!(
            parse_algorithm(&Some("RSA".into())).unwrap(),
            CryptoAlgorithm::RSA
        );
        assert_eq!(
            parse_algorithm(&Some("ECDSA".into())).unwrap(),
            CryptoAlgorithm::ECDSA
        );
    }

    #[test]
    fn returns_error_on_invalid_algorithm() {
        let err = parse_algorithm(&Some("SHA256".into())).unwrap_err();
        assert!(matches!(err, SecureError::PipelineError(_)));

        let none_case = parse_algorithm(&None).unwrap_err();
        assert!(matches!(none_case, SecureError::PipelineError(_)));
    }

    #[test]
    fn parses_key_length_from_algorithm_string() {
        assert_eq!(
            parse_key_length(&Some("AES-256".into())).unwrap(),
            KeyLength::Bits256
        );
        assert_eq!(
            parse_key_length(&Some("RSA".into())).unwrap(),
            KeyLength::Bits2048
        );
        assert_eq!(
            parse_key_length(&Some("ECDSA".into())).unwrap(),
            KeyLength::Bits256
        );
    }

    #[test]
    fn returns_error_on_invalid_key_length_request() {
        let err = parse_key_length(&Some("SHA1".into())).unwrap_err();
        assert!(matches!(err, SecureError::PipelineError(_)));

        let none_case = parse_key_length(&None).unwrap_err();
        assert!(matches!(none_case, SecureError::PipelineError(_)));
    }
}
