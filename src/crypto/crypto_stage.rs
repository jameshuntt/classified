#![cfg(feature = "async")]
//! 
//! FEATURE NOTES
//! 
//! feature_name: async
//! deps: tokio | async-trait
//! scope: impl PipelineStage
//! effected_lines: []
//! corpus: false 
//! 
//! feature_name: logging
//! deps: tracing
//! scope: function inside trait impl
//! effected_lines: []
//! corpus: false 
//! 
//! 

use crate::{crypto::crypto_primitive::CryptoPrimitive, traits::PipelineStage};
use crate::errors::SecureError;
use crate::classified_data::ClassifiedData;

#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "logging")]
use tracing::{info};

pub struct CryptoStage {
    crypto: CryptoPrimitive,
}

impl CryptoStage {
    pub fn new(crypto: CryptoPrimitive) -> Self {
        Self { crypto }
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl PipelineStage for CryptoStage {
    async fn process(&self, data: ClassifiedData<Vec<u8>>) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        #[cfg(feature = "logging")]
        info!("CryptoStage processing data.");
        
        let encrypted = self.crypto.encrypt(data.expose()).expect("Error");
        Ok(ClassifiedData::new(encrypted))
    }
}


