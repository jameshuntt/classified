//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:errors.rs
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

#![allow(unused)]

// Module for Error Handling

use std::io;

use thiserror::Error;



#[macro_export]
macro_rules! attach_error_to_hex {
    ($hex_code:expr, $variant_type:ty, $enum_variant:path) => {
        impl From<$variant_type> for $crate::errors::SecureError {
            fn from(err: $variant_type) -> Self {
                $crate::errors::SecureError::HexCodeError($crate::errors::HexError {
                    code: [
                        (($hex_code >> 16) & 0xFF) as u8,
                        (($hex_code >> 8) & 0xFF) as u8,
                        ($hex_code & 0xFF) as u8,
                        ],
                        // Stringify the type name for the message
                        message: core::stringify!($variant_type).to_string(),
                    })
                }
            }
        };
        ($hex_code:expr, $error_variant:path) => {
            impl From<$error_variant> for $crate::errors::SecureError {
                fn from(_: $error_variant) -> Self {
                    $crate::errors::SecureError::HexCodeError($crate::errors::HexError {
                        // We take the u32 and turn it into [u8; 3] 
                        // (taking the lower 3 bytes)
                        code: [
                            (($hex_code >> 16) & 0xFF) as u8,
                            (($hex_code >> 8) & 0xFF) as u8,
                            ($hex_code & 0xFF) as u8,
                        ],
                        message: core::stringify!($error_variant).to_string(),
                    })
                }
            }
        };
}

// Usage:
attach_error_to_hex!(0x1001, InvalidKeyLength, SecureError::InvalidKeyLength);

#[derive(core::fmt::Debug)]
pub struct InvalidKeyLength;
pub struct PipelineError(pub String);
#[derive(core::fmt::Debug)]
pub struct HexError
{code:[u8;3],message:String}



#[derive(Debug)]
pub enum SecureErrors {
    InvalidKeyLength(InvalidKeyLength),
    PipelineError(String),
    HexCodeError(HexError)
}

#[derive(Error, Debug)]
pub enum SecureError {
    #[error("Invalid key length specified")]
    InvalidKeyLength,
    #[error("Pipeline setup failed: {0}")]
    PipelineError(String),
    #[error("Hex code error")]
    HexCodeError(HexError)
}
#[derive(Error, Debug)]
pub enum ConcurrencyError {
    #[error("Invalid key length specified")]
    InvalidKeyLength,
    #[error("Pipeline setup failed: {0}")]
    PipelineError(String),
}

// Define specific error variants
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("AES Decryption failed")]
    AESDecryptionError,
    #[error("RSA Decryption failed")]
    RSADecryptionError,
    #[error("Decryption failed")]
    DecryptionError,

    #[error("AES Decryption failed")]
    AESEncryptionError,
    #[error("RSA Decryption failed")]
    RSAEncryptionError,
    #[error("Decryption failed")]
    EncryptionError,



    // #[error("Decryption failed")]
    // DecryptionError,
    // #[error("Decryption failed")]
    // DecryptionError,
    // #[error("Decryption failed")]
    // DecryptionError,
    // #[error("Decryption failed")]
    // DecryptionError,
    // Other crypto errors...
}


#[derive(Error, Debug)]
pub enum ClassifiedError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Hex code error {code}: {message}")]
    HexCodeError {
        code: u32,
        message: String,
    },
    #[error("Cryptographic error: {0}")]
    CryptographicError(String),
    #[error("Security error: {0}")]
    SecurityError(String),
    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),
    #[error("Pipeline error: {0}")]
    PipelineError(String),
    #[error("Cryptography error: {0}")]
    CryptoError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[cfg(feature = "std")]
    #[error("Serde error: {0}")]
    SerdeError(#[from] toml::de::Error),
    // Add more as needed
}

#[derive(Error, Debug)]
pub enum ClassifiedErrorMaster {
    #[error("Secure error: {0}")]
    SecureError(#[from] SecureError),

    #[error("Crypto error: {0}")]
    CryptoError(#[from] CryptoError),

    #[error("Other sensitive error: {0}")]
    Other(#[from] ClassifiedError),
}

impl Default for ClassifiedErrorMaster {
    fn default() -> Self {
        ClassifiedErrorMaster::SecureError(
            SecureError::InvalidKeyLength
        )
    }
}





// impl From<std::io::Error> for ClassifiedError {
//     fn from(error: std::io::Error) -> Self {
//         ClassifiedError::IoError(error.to_string())
//     }
// }
// 
// 




macro_rules! define_error {
    ($name:ident, $msg:expr) => {
        #[derive(thiserror::Error, Debug)]
        pub enum $name {
            #[error($msg)]
            CustomError,
        }
    };
}


macro_rules! attach_error_to_hex {
    ($code:expr, $error:expr) => {
        // Attach an error to a hexadecimal code for easy identification
        // Placeholder implementation for binding error codes
        // println!("Error code {}: {:?}", $code, $error);
    };
}


// Attach a hex code to the CryptoError variant
// Use the macro to attach hex codes
attach_error_to_hex!({0x1001u32},
    {ClassifiedErrorMaster::CryptoError(
        CryptoError::InvalidKeyLength
    )}
);


impl From<io::Error> for SecureError {
    fn from(e: io::Error) -> Self {
        SecureError::PipelineError(e.to_string())
    }
}

impl From<toml::de::Error> for SecureError {
    fn from(e: toml::de::Error) -> Self {
        SecureError::PipelineError(e.to_string())
    }
}


impl From<CryptoError> for SecureError {
    fn from(err: CryptoError) -> Self {
        SecureError::InvalidKeyLength  // or your variant name
    }
}
