//! 
//! FEATURE NOTES
//! 
//! feature_name: async
//! deps: tokio | async-trait
//! scope: struct ActorChannel
//! effected_lines: []
//! corpus: true 
//! 
//! feature_name: logging
//! deps: tracing
//! scope: function inside trait impl
//! effected_lines: []
//! corpus: false 
//! 
//! 








#![cfg(feature = "async")]

use async_trait::async_trait;
use tokio::sync::mpsc;

use super::csp::{
    CSPRecv,
    CSPSend,
    RecvError,
    SendError
};

pub struct ActorChannel<T> {
    pub sender: mpsc::Sender<T>,
    pub receiver: mpsc::Receiver<T>,
}

impl<T> ActorChannel<T> {
    pub fn new(bound: usize) -> Self {
        let (
            sender, 
            receiver
        ) = mpsc::channel(bound);

        Self { sender, receiver }
    }

    pub fn split(self) -> (
        ActorSender<T>,
        ActorReceiver<T>
    ) {
        (
            ActorSender { inner: self.sender },
            ActorReceiver { inner: self.receiver }
        )
    }
}

#[derive(Clone)]
pub struct ActorSender<T> {
    pub inner: mpsc::Sender<T>,
}

pub struct ActorReceiver<T> {
    pub inner: mpsc::Receiver<T>,
}

#[async_trait]
impl<T: Send + 'static> CSPSend<T> for ActorSender<T> {
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
impl<T: Send + 'static> CSPRecv<T> for ActorReceiver<T> {
    async fn recv(&mut self) -> Result<T, RecvError> {
        self
        .inner
        .recv()
        .await
        .ok_or_else(|| RecvError(
            "channel closed".into()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::csp::{CSPSend, CSPRecv};
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_actor_channel_send_recv_success() {
        let channel = ActorChannel::new(10);
        let (sender, mut receiver) = channel.split();

        let send_result = sender.send("hello").await;
        assert!(send_result.is_ok());

        let recv_result = receiver.recv().await;
        assert_eq!(recv_result.unwrap(), "hello");
    }

    #[tokio::test]
    async fn test_actor_channel_recv_error_when_closed() {
        let channel: ActorChannel<i32> = ActorChannel::new(1);
        let (sender, mut receiver) = channel.split();

        // Drop sender to simulate channel close
        drop(sender);

        // Attempt to receive should error
        let result = receiver.recv().await;
        assert!(result.is_err());
        // assert_eq!(result.unwrap_err().to_string(), "channel closed");
        assert_eq!(result.unwrap_err().to_string(), "RecvError: channel closed");

    }

    #[tokio::test]
    async fn test_actor_channel_clone_sender_multiple_sends() {
        let channel = ActorChannel::new(10);
        let (sender, mut receiver) = channel.split();

        let sender2 = sender.clone();

        sender.send(1).await.unwrap();
        sender2.send(2).await.unwrap();

        let mut received = vec![];
        received.push(receiver.recv().await.unwrap());
        received.push(receiver.recv().await.unwrap());

        received.sort();
        assert_eq!(received, vec![1, 2]);
    }

    #[tokio::test]
    async fn test_actor_channel_timeout_on_recv() {
        let channel: ActorChannel<i32> = ActorChannel::new(1);
        let (_sender, mut receiver) = channel.split();

        let result = timeout(Duration::from_millis(100), receiver.recv()).await;

        // We expect a timeout, because recv will wait forever with no sender
        assert!(result.is_err()); // timeout occurred
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "deadline has elapsed");
    }
}

use std::fmt;


impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RecvError: {}", self.0)
    }
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SendError: {}", self.0)
    }
}
