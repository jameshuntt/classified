//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:datd_repository.rs
//! description:
//! usages:none in crate yet
//! 
//! ----------------------------------------------
//! FEATURE NOTES --------------------------------
//! 
//! feature_name:async
//! deps:[tokio][async_trait]
//! scope:[impl ThreadPoolManager]
//! corpus:true
//! 
//! feature_name:std
//! deps:[std]
//! scope:[impl ThreadPoolManager]
//! corpus:false
//! 
//! ----------------------------------------------
//! CORPUS FEATURES ------------------------------
//! 
#![cfg(feature = "async")]
#![cfg(feature = "std")]

use crate::errors::SecureError;
use crate::classified_data::ClassifiedData;

pub async fn crypto_op_with_fallback_a<F, B>(
    strong: F,
    fallback: B,
) -> Result<ClassifiedData<Vec<u8>>, SecureError>
where
    F: std::future::Future<Output = Result<ClassifiedData<Vec<u8>>, SecureError>> + Send,
    B: FnOnce() -> ClassifiedData<Vec<u8>> + Send + 'static,
{
    let fallback_task = tokio::task::spawn_blocking(fallback);

    tokio::select! {
        result = strong => result,
        fallback = fallback_task => Ok(fallback.unwrap()),
    }
}

/// Attempts a strong crypto operation, but falls back to a local fallback function if needed.
pub async fn crypto_op_with_fallback_b<F, B>(
    strong_op: F,
    fallback: B,
) -> Result<ClassifiedData<Vec<u8>>, SecureError>
where
    F: std::future::Future<Output = Result<ClassifiedData<Vec<u8>>, SecureError>> + Send,
    B: FnOnce() -> ClassifiedData<Vec<u8>> + Send + 'static,
{
    let fallback_task = tokio::task::spawn_blocking(fallback);

    tokio::select! {
        result = strong_op => result,
        fallback = fallback_task => Ok(fallback.unwrap()),
    }
}
pub async fn crypto_op_with_fallback<F, B>(
    strong: F,
    fallback: B,
) -> Result<ClassifiedData<Vec<u8>>, SecureError>
where
    F: std::future::Future<Output = Result<ClassifiedData<Vec<u8>>, SecureError>> + Send,
    B: FnOnce() -> ClassifiedData<Vec<u8>> + Send + 'static,
{
    match strong.await {
        Ok(data) => Ok(data),
        Err(_) => {
            let fallback_data = tokio::task::spawn_blocking(fallback)
                .await
                .map_err(|e| SecureError::PipelineError(format!("Fallback task failed: {e}")))?;
            Ok(fallback_data)
        }
    }
}

#[tokio::test]
async fn test_crypto_fallback() {
    use crate::crypto_fallback::crypto_op_with_fallback;
    use crate::classified_data::ClassifiedData;

    let strong = async {
        Err(crate::errors::SecureError::PipelineError("Failing intentionally".into()))
    };

    let fallback = || ClassifiedData::new(vec![42; 16]);

    let result = crypto_op_with_fallback(strong, fallback).await;
    assert_eq!(result.unwrap().expose(), &[42; 16]);
}
