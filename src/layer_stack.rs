//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:layer_stack.rs
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




use crate::{
    errors::SecureError,
    traits::{
        Frame,
        FramedLayerHandler
    }
};

pub struct LayerStack {
    layers: Vec<Box<dyn FramedLayerHandler>>,
}

impl LayerStack {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn push<L: FramedLayerHandler + 'static>(&mut self, layer: L) {
        self.layers.push(Box::new(layer));
    }

    pub async fn run(&self, frame: Frame) -> Result<Frame, SecureError> {
        let mut current = frame;
        for layer in &self.layers {
            current = layer.handle(current).await?;
        }
        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{Frame, FramedLayerHandler};
    use crate::classified_data::ClassifiedData;
    use crate::errors::SecureError;
    use async_trait::async_trait;

    #[derive(Clone)]
    struct AppendLayer(&'static [u8]);

    #[async_trait]
    impl FramedLayerHandler for AppendLayer {
        async fn handle(&self, mut frame: Frame) -> Result<Frame, SecureError> {
            let mut data = frame.data.expose().clone();
            data.extend_from_slice(self.0);
            frame.data = ClassifiedData::new(data);
            Ok(frame)
        }
    }

    #[derive(Clone)]
    struct FailingLayer;

    #[async_trait]
    impl FramedLayerHandler for FailingLayer {
        async fn handle(&self, _frame: Frame) -> Result<Frame, SecureError> {
            Err(SecureError::PipelineError("intentional failure".into()))
        }
    }

    fn frame_with(data: &[u8]) -> Frame {
        Frame {
            data: ClassifiedData::new(data.to_vec()),
            metadata: None,
        }
    }

    #[tokio::test]
    async fn test_layer_stack_single_layer() {
        let mut stack = LayerStack::new();
        stack.push(AppendLayer(b"123"));

        let input = frame_with(b"abc");
        let result = stack.run(input).await.unwrap();

        assert_eq!(result.data.expose(), b"abc123");
    }

    #[tokio::test]
    async fn test_layer_stack_multiple_layers() {
        let mut stack = LayerStack::new();
        stack.push(AppendLayer(b"1"));
        stack.push(AppendLayer(b"2"));
        stack.push(AppendLayer(b"3"));

        let input = frame_with(b"start");
        let result = stack.run(input).await.unwrap();

        assert_eq!(result.data.expose(), b"start123");
    }

    #[tokio::test]
    async fn test_layer_stack_with_failure() {
        let mut stack = LayerStack::new();
        stack.push(AppendLayer(b"good"));
        stack.push(FailingLayer); // this one will fail
        stack.push(AppendLayer(b"should_not_run"));

        let input = frame_with(b"data");
        let result = stack.run(input).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SecureError::PipelineError(_)));
    }
}
