//! this file could qualify for no-std as long as i make a vec and hashmap no-std drop in
//!
//! this is a thing that is a naive thing that i made
//! 
//! i will replace it with mechanisms from pkcs11
//! 
//! there will be handling of all formats, legacy and otherwise
//! 
//! 



use secrecy::{SecretBox};
use zeroize::Zeroize;

use crate::errors::{CryptoError};
use super::{
    crypto_algorithm::CryptoAlgorithm,
    key_length::KeyLength
};



#[derive(Debug)]
pub struct CryptoPrimitive {
    pub algorithm: CryptoAlgorithm,
    pub key_length: KeyLength,
    pub key_material: SecretBox<Vec<u8>>,
    pub zeroize: bool,
}
impl CryptoPrimitive {
    pub fn new(algorithm: &CryptoAlgorithm, key_material: Vec<u8>, zeroize: bool) -> Result<Self, CryptoError> {
        let key_length = match algorithm {
            CryptoAlgorithm::AES => KeyLength::Bits256,
            CryptoAlgorithm::RSA => KeyLength::Bits2048,
            CryptoAlgorithm::ECDSA => KeyLength::Bits256,
        };

        if key_material.len() != key_length.as_bytes() {
            return Err(CryptoError::CryptoError("Key length mismatch".to_string()));
        }

        Ok(Self {
            algorithm: algorithm.clone(),
            key_length,
            key_material: SecretBox::new(Box::new(key_material)),
            zeroize,
        })
    }
}

impl CryptoPrimitive {
    // Implement encryption, decryption, etc.
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        match self.algorithm {
            CryptoAlgorithm::AES => self.aes_encrypt(data),
            CryptoAlgorithm::RSA => self.rsa_encrypt(data),
            CryptoAlgorithm::ECDSA => Err(CryptoError::CryptoError(
                "ECDSA is for signing, not encryption".into(),
            )),
        }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        match self.algorithm {
            CryptoAlgorithm::AES => self.aes_decrypt(data),
            CryptoAlgorithm::RSA => self.rsa_decrypt(data),
            CryptoAlgorithm::ECDSA => Err(CryptoError::CryptoError(
                "ECDSA is for verification, not decryption".into(),
            )),
        }
    }

    fn aes_encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // AES is symmetric; decryption is same as encryption
        Ok(data.to_vec())
    }

    fn aes_decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // AES is symmetric; decryption is same as encryption
        self.aes_encrypt(data)
    }

    fn rsa_encrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // Implement RSA encryption logic using a suitable crate like `rsa`
        // Placeholder implementation
        Ok(data.to_vec())
    }
    fn rsa_decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // Implement RSA decryption logic
        // Placeholder implementation
        Ok(data.to_vec())
    }
}

impl Drop for CryptoPrimitive {
    fn drop(&mut self) {
        if self.zeroize {
            self.key_material.zeroize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::crypto_algorithm::CryptoAlgorithm;

    #[test]
    fn creates_aes_primitive_with_valid_key() {
        let key = vec![0u8; 32]; // AES-256
        let result = CryptoPrimitive::new(&CryptoAlgorithm::AES, key, true);
        assert!(result.is_ok());
    }

    #[test]
    fn errors_on_invalid_key_length() {
        let key = vec![0u8; 10]; // Invalid for AES
        let result = CryptoPrimitive::new(&CryptoAlgorithm::AES, key, true);
        assert!(result.is_err());
    }

    #[test]
    fn aes_encrypt_and_decrypt_is_symmetric() {
        let key = vec![1u8; 32];
        let primitive = CryptoPrimitive::new(&CryptoAlgorithm::AES, key, false).unwrap();

        let plaintext = b"secret data";
        let encrypted = primitive.encrypt(plaintext).unwrap();
        assert_ne!(encrypted, plaintext);

        let decrypted = primitive.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn rsa_encrypt_decrypt_is_passthrough() {
        let key = vec![2u8; 256];
        let primitive = CryptoPrimitive::new(&CryptoAlgorithm::RSA, key, false).unwrap();

        let data = b"just testing";
        let enc = primitive.encrypt(data).unwrap();
        assert_eq!(enc, data);

        let dec = primitive.decrypt(&enc).unwrap();
        assert_eq!(dec, data);
    }

    #[test]
    fn ecdsa_encrypt_fails() {
        let key = vec![3u8; 32];
        let primitive = CryptoPrimitive::new(&CryptoAlgorithm::ECDSA, key, false).unwrap();

        let data = b"can't encrypt this";
        let result = primitive.encrypt(data);
        assert!(result.is_err());
    }

    #[test]
    fn ecdsa_decrypt_fails() {
        let key = vec![3u8; 32];
        let primitive = CryptoPrimitive::new(&CryptoAlgorithm::ECDSA, key, false).unwrap();

        let data = b"can't decrypt this";
        let result = primitive.decrypt(data);
        assert!(result.is_err());
    }
}


