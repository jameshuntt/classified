#![cfg(feature = "async")]
//!
//! this uses tokio and async-trait, therefore it is async
//! 
//! 
//! 



use crate::errors::SecureError;
use crate::classified_data::ClassifiedData;
use crate::traits::PipelineStage;

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use std::time::Instant;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
#[cfg(feature = "logging")]
use tracing::{debug, error, warn};

// OTEL metrics
use opentelemetry::{global, KeyValue};
use opentelemetry::metrics::{Counter, Histogram};

// Initialization (can go in your main or setup)
fn init_meter() -> (Counter<u64>, Counter<u64>, Counter<u64>, Histogram<f64>) {
    let meter = global::meter("stream_handler");

    let success_counter = meter.u64_counter("process_success").build();
    let failure_counter = meter.u64_counter("process_failure").build();
    let stream_failure_counter = meter.u64_counter("stream_failure").build();
    let duration_histogram = meter.f64_histogram("process_duration").build();

    (
        success_counter,
        failure_counter,
        stream_failure_counter,
        duration_histogram,
    )
}

type SecureStreamData = ClassifiedData<Vec<u8>>;
type SecureStreamResult = Result<SecureStreamData, SecureError>;

#[derive(Clone)]
pub struct StreamHandler {
    concurrency_limit: usize,
    max_retries: usize,
    success_counter: Counter<u64>,
    failure_counter: Counter<u64>,
    stream_failure_counter: Counter<u64>,
    duration_histogram: Histogram<f64>,
}

impl StreamHandler {
    pub fn new(concurrency_limit: usize, max_retries: usize) -> Self {
        let (success, failure, stream_failure, duration) = init_meter();

        Self {
            concurrency_limit,
            max_retries,
            success_counter: success,
            failure_counter: failure,
            stream_failure_counter: stream_failure,
            duration_histogram: duration,
        }
    }
    #[cfg(feature = "std")]

    pub async fn run_stream<S>(&self, stream: S) -> Result<(), SecureError>
    where
        S: Stream<Item = SecureStreamResult> + Unpin + Send + 'static,
    {
        stream
            .for_each_concurrent(Some(self.concurrency_limit), |item| {
                let this = self.clone();
                async move { this.handle_item(item).await }
            })
            .await;

        Ok(())
    }

    #[cfg(feature = "std")]
    async fn handle_item(
        &self,
        item: SecureStreamResult,
    ) {
        match item {
            Ok(data) => self.handle_data_with_retries(data).await,
            Err(e) => {
                #[cfg(feature = "logging")]
                error!("Stream item error: {}", e);
                self.stream_failure_counter.add(1, &[]);
            }
        }
    }
    #[cfg(feature = "std")]
    async fn handle_data_with_retries(
        &self,
        data: SecureStreamData,
    ) {
        let mut retries = 0;
        loop {
            let start = Instant::now();

            match self.process_data(data.clone()).await {
                Ok(_) => {
                    self.success_counter.add(1, &[KeyValue::new("stage", "ingest")]);
                    self.duration_histogram.record(start.elapsed().as_secs_f64(), &[]);
                    break;
                }
                Err(e) => {
                    retries += 1;
                    #[cfg(feature = "logging")]
                    warn!("Retry {}/{} failed: {}", retries, self.max_retries, e);
                    if retries > self.max_retries {
                        #[cfg(feature = "logging")]
                        error!("Max retries exceeded for item: {}", e);
                        self.failure_counter.add(1, &[]);
                        break;
                    }
                }
            }
        }
    }

    async fn process_data(&self, data: SecureStreamData) -> Result<(), SecureError> {
        #[cfg(feature = "logging")]
        debug!("Processing stream item: {:?}", data.expose());

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        Ok(())
    }
}

#[async_trait]
impl PipelineStage for StreamHandler {
    async fn process(&self, data: SecureStreamData) -> SecureStreamResult {
        self.process_data(data.clone()).await?;
        Ok(data)
    }
}


pub fn create_data_stream() -> impl Stream<Item = SecureStreamResult> {
    let (sender, receiver) = mpsc::channel::<SecureStreamResult>(32);

    tokio::spawn(async move {
        for i in 0..10 {
            let payload = format!("Streamed Data {i}").as_bytes().to_vec();
            let item = ClassifiedData::new(payload);
            if let Err(e) = sender.send(Ok(item)).await {
                #[cfg(feature = "logging")]
                error!("Failed to send stream item: {}", e);
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    ReceiverStream::new(receiver)
}


pub fn create_stream(stage_config: &crate::config::StageConfig) -> Result<impl PipelineStage, SecureError> {
    let concurrency = stage_config.concurrency_limit.unwrap_or(4);
    let retries = stage_config.max_retries.unwrap_or(3);
    Ok(StreamHandler::new(concurrency, retries))
}

    #[cfg(feature = "std")]

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_stream_handler_ok() {
        let handler = StreamHandler::new(4, 2);

        let (sender, receiver) = mpsc::channel::<SecureStreamResult>(10);
        let stream = ReceiverStream::new(receiver);

        tokio::spawn(async move {
            for i in 0..3 {
                let data = ClassifiedData::new(vec![i; 4]);
                sender.send(Ok(data)).await.unwrap();
            }
        });

        let result = handler.run_stream(stream).await;
        assert!(result.is_ok());
    }
}

