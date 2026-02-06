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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum KeyLength {
    Bits256,
    Bits512,
    Bits1024,
    #[default]
    Bits2048,
    Bits4096,
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
