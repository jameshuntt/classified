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

use crate::config::{PipelineConfig, StageConfig};
use crate::crypto::crypto_algorithm::CryptoAlgorithm;
use crate::crypto::crypto_primitive::CryptoPrimitive;
use crate::errors::SecureError;
use crate::classified_data::ClassifiedData;
// use crate::pipeline_builder::{PipelineStage, create_actor, create_stream, create_future, create_csp};
use crate::traits::PipelineStage;
use std::sync::Arc;

pub struct Pipeline {
    stages: Vec<Arc<dyn PipelineStage>>,
}

impl Pipeline {
    pub async fn initialize(config: &PipelineConfig) -> Result<Self, SecureError> {
        let mut stages = Vec::new();

        for stage_config in &config.stages {
            if !stage_config.enabled {
                continue;
            }

            let stage: Arc<dyn PipelineStage> = match stage_config.stage_type.to_string().as_str() {
                "actor" => Arc::new(create_actor(stage_config)?),
                "stream" => Arc::new(create_stream(stage_config)?),
                "future" => Arc::new(create_future(stage_config)?),
                "csp" => Arc::new(create_csp(stage_config)?),
                _ => return Err(SecureError::PipelineError(format!("Unknown stage: {}", stage_config.stage_type))),
            };

            stages.push(stage);
        }

        Ok(Pipeline { stages })
    }

    pub async fn run(&self, mut data: ClassifiedData<Vec<u8>>) -> Result<(), SecureError> {
        for stage in &self.stages {
            data = stage.process(data).await?;
        }
        Ok(())
    }

    pub fn add_stage<S: PipelineStage + 'static>(&mut self, stage: S) {
        self.stages.push(Arc::new(stage));
    }
}

// #[async_trait]
// pub trait PipelineStage: Send + Sync {
//     async fn process(
//         &self,
//         data: ClassifiedData<Vec<u8>>
//     ) -> Result<ClassifiedData<Vec<u8>>, SecureError>;
// }

pub fn create_actor(config: &StageConfig) -> Result<impl PipelineStage, SecureError> {
    let algorithm_str = config.algorithm
        .clone()
        .ok_or_else(|| SecureError::PipelineError(
            "algorithm".to_owned()
        ))?;

    let algorithm = match algorithm_str.as_str() {
        "AES" => CryptoAlgorithm::AES,
        "RSA" => CryptoAlgorithm::RSA,
        "ECDSA" => CryptoAlgorithm::ECDSA,
        other => return Err(SecureError::PipelineError(format!("Unsupported algorithm: {}", other))),
    };

    let key_material = config.key_material.clone()
        .ok_or_else(|| SecureError::PipelineError(
            "Missing key material".to_string()
        ))?;

    let crypto = CryptoPrimitive::new(
        &algorithm,
        key_material,
        true
    )
        .map_err(|e| SecureError::PipelineError(
            "Crypto error".to_string()
        ))?;

    Ok(crate::concurrency::actors::EncryptionActor::new(crypto, None))
}

pub fn create_stream(config: &StageConfig) -> Result<impl PipelineStage, SecureError> {
    Ok(crate::concurrency::streams::StreamHandler::new(
        config.concurrency_limit.unwrap_or(4),
        config.max_retries.unwrap_or(3),
    ))
}

pub fn create_future(config: &StageConfig) -> Result<impl PipelineStage, SecureError> {
    Ok(crate::concurrency::futures::FutureHandler::new(config.concurrency_limit.unwrap_or(10)))
}

pub fn create_csp(config: &StageConfig) -> Result<impl PipelineStage, SecureError> {
    Ok(crate::concurrency::csp::CSPChannel::new(config.buffer_size.unwrap_or(32)))
}

// use sensitive::{pipeline::Pipeline, config::PipelineConfig, ClassifiedData};
// 
// #[tokio::main]
// async fn main() -> Result<(), SecureError> {
//     let config: PipelineConfig = load_config();
//     let pipeline = Pipeline::initialize(&config).await?;
// 
//     let data = ClassifiedData::new(vec![1, 2, 3, 4, 5]);
//     pipeline.run(data).await?;
// 
//     Ok(())
// }
// 