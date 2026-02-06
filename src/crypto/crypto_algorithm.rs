// src/crypto/crypto_algorithm.rs
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoAlgorithm {
    RSA,
    ECDSA,
    AES,
}
