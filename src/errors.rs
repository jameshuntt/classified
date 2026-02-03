#![allow(unused)]

// Module for Error Handling

use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecureError {
    #[error("Invalid key length specified")]
    InvalidKeyLength,
    #[error("Pipeline setup failed: {0}")]
    PipelineError(String),
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
pub enum SensitiveError {
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
    Other(#[from] SensitiveError),
}

impl Default for ClassifiedErrorMaster {
    fn default() -> Self {
        ClassifiedErrorMaster::SecureError(
            SecureError::InvalidKeyLength
        )
    }
}





// impl From<std::io::Error> for SensitiveError {
//     fn from(error: std::io::Error) -> Self {
//         SensitiveError::IoError(error.to_string())
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
