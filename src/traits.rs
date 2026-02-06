//! ----------------------------------------------
//! DOCUMENT DETAILS -----------------------------
//! 
//! filename:traits.rs
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

pub trait ClassifiedEq<Rhs = Self> {
    fn classified_eq(&self, rhs: &Rhs) -> bool;
}

#[allow(unused)]trait KeyType:Eq+std::hash::Hash+Clone+std::fmt::Debug{}
#[allow(unused)]trait ValueType:zeroize::Zeroize+Clone+std::fmt::Debug{}


use crate::{
    errors::SecureError,
    classified_data::ClassifiedData
};

pub trait LayerHandler: Send + Sync {
    fn handle(
        &self,
        frame: ClassifiedData<Vec<u8>>
    ) -> Result<ClassifiedData<Vec<u8>>, SecureError>;
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub data: ClassifiedData<Vec<u8>>,
    pub metadata: Option<String>,
}


#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "async")]
#[async_trait]
pub trait PipelineStage: 'static + Send + Sync {
    async fn process(
        &self,
        data: ClassifiedData<Vec<u8>>,
    ) -> Result<ClassifiedData<Vec<u8>>, SecureError>;
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait FramedLayerHandler: Send + Sync {
    async fn handle(
        &self,
        frame: Frame
    ) -> Result<Frame, SecureError>;
}
