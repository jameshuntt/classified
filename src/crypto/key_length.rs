// src/crypto/key_length.rs
#[derive(Debug, Clone, PartialEq)]
pub enum KeyLength {
    Bits1024,
    Bits2048,
    Bits4096,
    Bits256, // For AES-256
    Bits512, // For AES-512, if applicable
}

impl KeyLength {
    pub fn as_bytes(&self) -> usize {
        match self {
            KeyLength::Bits1024 => 1024 / 8,
            KeyLength::Bits2048 => 2048 / 8,
            KeyLength::Bits4096 => 4096 / 8,
            KeyLength::Bits256 => 256 / 8,
            KeyLength::Bits512 => 512 / 8,
        }
    }
}
