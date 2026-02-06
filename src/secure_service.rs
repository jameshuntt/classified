use crate::errors::SecureError;
use crate::key_manager::KeyManager;
use crate::pipelines::Pipeline;
use crate::classified_data::ClassifiedData;
use std::sync::Arc;

pub struct SecureService {
    pipeline: Arc<Pipeline>,
    key_manager: Arc<KeyManager>,
}

impl SecureService {
    pub fn new(pipeline: Arc<Pipeline>, key_manager: Arc<KeyManager>) -> Self {
        Self {
            pipeline,
            key_manager,
        }
    }

    pub async fn handle_data(&self, data: Vec<u8>) -> Result<(), SecureError> {
        let sensitive_data = ClassifiedData::new(data);
        self.pipeline.run(sensitive_data).await
    }

    pub async fn add_key(&self, id: String, key: Vec<u8>) -> Result<(), SecureError> {
        self.key_manager.accept_key(&id, key.len(), key).await
    }

    pub async fn get_key(&self, id: &str) -> Option<ClassifiedData<Vec<u8>>> {
        self.key_manager.get_key(id).await
    }
    
    pub async fn delete_key(&self, id: &str) -> Result<(), SecureError> {
        self.key_manager.remove_key(id).await
    }
}
