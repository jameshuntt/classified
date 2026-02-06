#![cfg(feature = "async")]
//!
//! this uses tokio and async-trait, therefore it is async
//! 
//! 
//! 



use crate::{
    errors::SecureError,
    traits::PipelineStage,
    classified_data::ClassifiedData
};

use async_trait::async_trait;
use std::sync::Arc;
use tokio::{sync::{mpsc, Mutex}, time::{timeout, Duration}};
use tracing::{info, error};

use std::fmt::Debug;

#[async_trait]
pub trait CSPSend<T>: Send + Sync where T: Send + 'static, {
    async fn send(&self, data: T) -> Result<(), SendError>;
}

#[async_trait]
pub trait CSPRecv<T>: Send + Sync where T: Send + 'static, {
    async fn recv(&mut self) -> Result<T, RecvError>;
}

#[derive(Debug)]
pub struct SendError(pub String);

#[derive(Debug)]
pub struct RecvError(pub String);


pub struct CSPChannel {
    sender: mpsc::Sender<ClassifiedData<Vec<u8>>>,
    receiver: Arc<Mutex<mpsc::Receiver<ClassifiedData<Vec<u8>>>>>,
}

pub struct MpscSender<T> {
    inner: mpsc::Sender<T>,
}

pub struct MpscReceiver<T> {
    inner: mpsc::Receiver<T>,
}

#[async_trait]
impl<T: Send + 'static> CSPSend<T> for MpscSender<T> {
    async fn send(&self, data: T) -> Result<(), SendError> {
        self.inner
            .send(data)
            .await
            .map_err(|e| SendError(
                e.to_string()
            ))
    }
}

#[async_trait]
impl<T: Send + 'static> CSPRecv<T> for MpscReceiver<T> {
    async fn recv(&mut self) -> Result<T, RecvError> {
        self.inner
            .recv()
            .await
            .ok_or_else(|| RecvError(
                "channel closed".into()
            ))
    }
}

impl CSPChannel {
    pub fn new(buffer: usize) -> Self {
        let (sender, receiver) = mpsc::channel(buffer);
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<ClassifiedData<Vec<u8>>> {
        self.sender.clone()
    }

    pub fn get_receiver(&self) -> Arc<Mutex<mpsc::Receiver<ClassifiedData<Vec<u8>>>>> {
        Arc::clone(&self.receiver)
    }
}

#[async_trait]
impl PipelineStage for CSPChannel {
    async fn process(&self, data: ClassifiedData<Vec<u8>>) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        // Send
        match timeout(Duration::from_secs(5), self.sender.send(data)).await {
            Ok(Ok(())) => {
                info!("CSPChannel: sent successfully.");
            },
            Ok(Err(e)) => {
                error!("CSPChannel send failed: {}", e);
                return Err(SecureError::PipelineError(e.to_string()));
            },
            Err(_) => {
                error!("CSPChannel send timeout.");
                return Err(SecureError::PipelineError("Send timeout".into()));
            }
        }

        // Receive
        let mut rx = self.receiver.lock().await;
        match timeout(Duration::from_secs(5), rx.recv()).await {
            Ok(Some(data)) => {
                info!("CSPChannel: received successfully.");
                Ok(data)
            },
            Ok(None) => {
                error!("CSPChannel: channel closed unexpectedly.");
                Err(SecureError::InvalidKeyLength)
            },
            Err(_) => {
                error!("CSPChannel receive timeout.");
                Err(SecureError::PipelineError("Receive timeout".into()))
            }
        }
    }
}
