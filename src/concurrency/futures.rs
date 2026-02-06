use crate::errors::SecureError;
use crate::classified_data::ClassifiedData;
use crate::traits::PipelineStage;
use async_trait::async_trait;
use std::{sync::Arc};
use tokio::sync::Semaphore;
#[cfg(feature = "logging")]
use tracing::{debug, error, info, warn};
use std::time::Instant;
use tokio::time::{timeout, Duration};

pub struct FutureHandler {
    semaphore: Arc<Semaphore>,
    timeout_duration: Duration,
}

impl FutureHandler {
    pub fn new(concurrency_limit: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(concurrency_limit)),
            timeout_duration: Duration::from_secs(10),
        }
    }

    async fn process_data(&self, data: ClassifiedData<Vec<u8>>) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        #[cfg(feature = "logging")]
        debug!("FutureHandler: starting processing of {:?}", data);

        if data.is_empty() {
            #[cfg(feature = "logging")]
            warn!("FutureHandler: empty data provided.");
            return Err(SecureError::PipelineError("Empty data provided".into()));
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
        Ok(data)
    }

    pub async fn execute_future(&self, data: ClassifiedData<Vec<u8>>) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        let _permit = self.semaphore.clone().acquire_owned().await
            .map_err(|_| SecureError::PipelineError("Semaphore closed".into()))?;

        let start = Instant::now();
        #[cfg(feature = "logging")]
        info!("FutureHandler: executing future with timeout {:?}", self.timeout_duration);

        let result = timeout(self.timeout_duration, self.process_data(data)).await;

        match result {
            Ok(Ok(res)) => {
                #[cfg(feature = "logging")]
                info!("FutureHandler: completed in {:?}", start.elapsed());
                Ok(res)
            }
            Ok(Err(e)) => {
                #[cfg(feature = "logging")]
                error!("FutureHandler: processing error {:?}", e);
                Err(e)
            }
            Err(_) => {
                #[cfg(feature = "logging")]
                error!("FutureHandler: timed out after {:?}", self.timeout_duration);
                Err(SecureError::PipelineError("Processing timed out".into()))
            }
        }
    }
}

#[async_trait]
impl PipelineStage for FutureHandler {
    async fn process(
        &self,
        data: ClassifiedData<Vec<u8>>
    ) -> Result<ClassifiedData<Vec<u8>>, SecureError> {
        self.execute_future(data).await
    }
}

