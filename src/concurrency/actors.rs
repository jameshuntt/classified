//! 
//! FEATURE NOTES
//! 
//! feature_name: async
//! deps:[tokio][async_trait]
//! scope: [
//!     a: [struct ActorRef]
//!     b: [struct EncryptedActor]
//! ]
//! effected_lines: [
//!     a: 
//!     b: 
//! ]
//! corpus: true 
//! 
//! feature_name: logging
//! deps: [tracing]
//! scope: [
//!     ActorRef::new()
//!     EncryptedActor::start()
//! ]
//! effected_lines: []
//! corpus: false 
//! 
//! 








#![cfg(feature = "async")]


use std::sync::Arc;

use tokio::sync::{
    mpsc,
    mpsc::{
        Receiver,
        Sender
    },
    watch
};

use crate::{
    config::StageConfig,
    crypto::{
        crypto_algorithm::CryptoAlgorithm,
        crypto_primitive::CryptoPrimitive
    },
    errors::{
        CryptoError, SecureError, ClassifiedError
    },
    classified_data::ClassifiedData,
    traits::PipelineStage
};

type SecureSender   = Sender<ClassifiedData<Vec<u8>>>;
type SecureReceiver = Receiver<ClassifiedData<Vec<u8>>>;
// extern crate proc_macro;


#[async_trait::async_trait]
pub trait Actor: Send + Sync {
    async fn handle(
        &self,
        msg: ClassifiedData<Vec<u8>>
    ) -> Result<ClassifiedData<Vec<u8>>, SecureError>;
}

pub struct EncryptionActor {
    crypto: CryptoPrimitive,
    next: Option<mpsc::Sender<ClassifiedData<Vec<u8>>>>,
}

impl EncryptionActor {
    pub fn new(crypto: CryptoPrimitive, next: Option<mpsc::Sender<ClassifiedData<Vec<u8>>>>) -> Self {
        Self { crypto, next }
    }

    pub fn start(
        self: Arc<Self>,
        mut receiver: mpsc::Receiver<ClassifiedData<Vec<u8>>>,
        mut shutdown_rx: watch::Receiver<bool>
    ) {
        let actor = Arc::clone(&self);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx.changed() => {
                        if *shutdown_rx.borrow() {
                            #[cfg(feature = "logging")]
                            tracing::info!("EncryptionActor shutting down.");
                            break;
                        }
                    }
                    Some(message) = receiver.recv() => {
                        match actor.handle(message).await {
                            Ok(encrypted) => {
                                if let Some(ref next_sender) = actor.next {
                                    if let Err(e) = next_sender.send(encrypted).await {
                                        #[cfg(feature = "logging")]
                                        tracing::error!("Failed to forward encrypted data: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                #[cfg(feature = "logging")]
                                tracing::error!("Encryption error: {}", e);
                            }
                        }
                    }
                    else => break,
                }
            }
        });
    }
}

#[async_trait::async_trait]
impl Actor for EncryptionActor {
    async fn handle(
        &self,
        msg: ClassifiedData<Vec<u8>>
    ) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        let encrypted = self.crypto.encrypt(msg.expose())?;
        // let encrypted = self.crypto
        //     .encrypt(msg.expose())
        //     .map_err(SecureError::InvalidKeyLength("".to_s))?;  // wrap CryptoError in SecureError

        Ok(ClassifiedData::new(encrypted))
    }
}

pub struct ActorRef {
    sender: mpsc::Sender<ClassifiedData<Vec<u8>>>,
    shutdown_tx: watch::Sender<bool>,
    handle: tokio::task::JoinHandle<()>,
}

impl ActorRef {
    // pub fn new<A: Actor + 'static>(actor: Arc<A>, queue_size: usize) -> Self {
    pub fn new<A: Actor + ?Sized + 'static>(actor: Arc<A>, queue_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(queue_size);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        // shadow as mutable so `recv`/`changed` will compile:
        let mut receiver = receiver;
        let mut shutdown_rx = shutdown_rx;
        
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx.changed() => {
                        if *shutdown_rx.borrow() {
                            break;
                        }
                    }
                    Some(msg) = receiver.recv() => {
                        if let Err(e) = actor.handle(msg).await {
                            #[cfg(feature = "logging")]
                            tracing::error!("Actor failed: {e}");
                        }
                    }
                    else => break,
                }
            }
        });

        Self {
            sender,
            shutdown_tx,
            handle: task,
        }
    }

    pub async fn send(&self, msg: ClassifiedData<Vec<u8>>) -> Result<(), ClassifiedError> {
        self.sender
            .send(msg)
            .await
            .map_err(|e| ClassifiedError::ConcurrencyError(
                e.to_string()
            ))
    }

    pub async fn shutdown(self) {
        let _ = self.shutdown_tx.send(true);
        let _ = self.handle.await;
    }
}

#[async_trait::async_trait]
impl PipelineStage for EncryptionActor {
    async fn process(&self, data: ClassifiedData<Vec<u8>>) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        self.handle(data).await
    }
}


pub fn create_actor(
    cfg: &StageConfig
) -> Result<Arc<dyn PipelineStage>, SecureError> {
    if let Some(algo) = &cfg.algorithm {
        let algo = match algo.as_str() {
            "AES" => CryptoAlgorithm::AES,
            "RSA" => CryptoAlgorithm::RSA,
            "ECDSA" => CryptoAlgorithm::ECDSA,
            _ => return Err(SecureError::PipelineError(
                format!("Unsupported algorithm: {}", algo)
            )),
        };
        
        let key_material = cfg.key_material.clone()
            .ok_or_else(|| SecureError::PipelineError(
                "Missing key material".into()
            ))?;
        
        let zeroize = cfg.zeroize.unwrap_or(true);
        
        let crypto = CryptoPrimitive::new(
            &algo,
            key_material,
            zeroize
        )
            .map_err(|e| SecureError::PipelineError(
                format!("Init failed: {}", e)
            ))?;
    
        Ok(Arc::new(EncryptionActor::new(crypto, None)))
    } else {
        Err(SecureError::PipelineError("Missing algorithm".into()))
    }
}
#[async_trait::async_trait]
impl Actor for dyn PipelineStage {
    async fn handle(&self, msg: ClassifiedData<Vec<u8>>) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        self.process(msg).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        // config::StageConfig,
        crypto::{crypto_algorithm::CryptoAlgorithm, crypto_primitive::CryptoPrimitive},
        classified_data::ClassifiedData,
    };
    use std::sync::Arc;
    use tokio::{
        sync::{mpsc, watch},
        time::{timeout, Duration}
    };

    // fn valid_config() -> StageConfig {
    //     StageConfig {
    //         algorithm: Some("AES".to_string()),
    //         key_material: Some(vec![0u8; 32]),
    //         zeroize: Some(true),
    //         ..Default::default()
    //     }
    // }
    use crate::config::{StageConfig, StageType};

    fn test_stage_config() -> StageConfig {
        StageConfig {
            name: "encryption".to_string(),
            stage_type: StageType::Actor, // or whatever variant is correct
            // stage_type: StageType::Crypto, // or whatever variant is correct
            algorithm: Some("AES".to_string()),
            enabled: true,
            concurrency_limit: Some(4),
            max_retries: Some(3),
            buffer_size: Some(16),
            key_material: Some(vec![0x00; 32]), // example 256-bit key
            zeroize: Some(true),
        }
    }


    #[tokio::test]
    async fn test_create_actor_success() {
        // let cfg = valid_config();
        let cfg = test_stage_config();
        let result = create_actor(&cfg);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_actor_missing_algorithm() {
        // let mut cfg = valid_config();
        let mut cfg = test_stage_config();

        cfg.algorithm = None;
        let result = create_actor(&cfg);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_actor_invalid_key() {
        // let mut cfg = valid_config();
        let mut cfg = test_stage_config();

        cfg.key_material = Some(vec![1, 2, 3]); // invalid length
        let result = create_actor(&cfg);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_encryption_actor_handle() {
        // let cfg = valid_config();
        let cfg = test_stage_config();
        
        let actor = create_actor(&cfg).unwrap();
        let data = ClassifiedData::new(b"secret".to_vec());
        let result = actor.process(data);
        assert!(result.await.is_ok());
    }

    #[tokio::test]
    async fn test_encryption_actor_forwarding() {
        // let cfg = valid_config();
        let cfg = test_stage_config();

        let primitive = CryptoPrimitive::new(
            &CryptoAlgorithm::AES,
            cfg.key_material.clone().unwrap(),
            true,
        )
        .unwrap();

        let (tx, mut rx) = mpsc::channel(1);
        let actor = Arc::new(EncryptionActor::new(primitive, Some(tx)));

        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let data = ClassifiedData::new(b"hello".to_vec());

        actor.clone().start(rx, shutdown_rx);

        let result = actor.handle(data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_actor_ref_send_and_shutdown() {
        // let cfg = valid_config();
        let cfg = test_stage_config();

        let actor = create_actor(&cfg).unwrap();
        let actor_ref = ActorRef::new(actor.clone(), 4);

        let result = actor_ref.send(ClassifiedData::new(b"data".to_vec())).await;
        assert!(result.is_ok());

        actor_ref.shutdown().await;
    }

    #[tokio::test]
    async fn test_actor_ref_error_on_shutdown_channel() {
        // let cfg = valid_config();
        let cfg = test_stage_config();

        let actor = create_actor(&cfg).unwrap();
        let actor_ref = ActorRef::new(actor.clone(), 4);

        let result = actor_ref.send(ClassifiedData::new(b"1234".to_vec())).await;
        assert!(result.is_ok());

        // Drop sender early and ensure shutdown doesn't panic
        drop(actor_ref.sender.clone());
        actor_ref.shutdown().await;
    }

    #[tokio::test]
    async fn test_actor_ref_timed_encryption_loop() {
        // let cfg = valid_config();
        let cfg = test_stage_config();

        let actor = create_actor(&cfg).unwrap();
        let actor_ref = ActorRef::new(actor.clone(), 4);
        let result = timeout(
            Duration::from_secs(1),
            actor_ref.send(ClassifiedData::new(b"data".to_vec())),
        )
        .await;
        assert!(result.is_ok());
        actor_ref.shutdown().await;
    }
}
