#![allow(unused)]

use serde::Deserialize;
// use crate::config::Config;
use std::{fs, sync::LazyLock};

use crate::{
    errors::{
        SecureError,
        ClassifiedError
    },
    classified_data::ClassifiedData
};


#[derive(Clone, Deserialize)]
pub struct ProtocolsConfig {}


#[derive(Clone, Deserialize)]
pub struct Config {
    pub security: SecurityConfig,
    pub concurrency: ConcurrencyConfig,
    pub pipeline: PipelineConfig,
    pub protocols: Option<ProtocolsConfig>,
}

#[derive(Deserialize, Clone)]
pub struct SecurityConfig {
    pub enable_zeroize: bool,
}




#[derive(Deserialize, Clone)]
pub struct PipelineConfig {
    pub stages: Vec<StageConfig>,
}

#[derive(Deserialize, Clone)]
pub struct StageConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub stage_type: StageType,
    pub algorithm: Option<String>,
    pub enabled: bool, // To conditionally enable/disable stages
    
    // Add these:
    pub concurrency_limit: Option<usize>,
    pub max_retries: Option<usize>,
    pub buffer_size: Option<usize>,
    pub key_material: Option<Vec<u8>>, // ← Add this
    pub zeroize: Option<bool>,         // ← Maybe add this too
}


#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StageType {
    Actor,
    Stream,
    Future,
    CSP,
}

use std::fmt;

impl fmt::Display for StageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}







#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ConcurrencyPattern {
    Actors,
    Streams,
    Futures,
    CSP,
}

#[derive(Deserialize, Clone)]
pub struct ConcurrencyConfig {
    pub pattern: ConcurrencyPattern,
}

impl ConcurrencyConfig {
    fn is_valid(&self) -> bool {
        matches!(
            self.pattern,
            ConcurrencyPattern::Actors
                | ConcurrencyPattern::Streams
                | ConcurrencyPattern::Futures
                | ConcurrencyPattern::CSP
        )
    }
}






use serde::Deserializer;

impl<'de, T> Deserialize<'de> for ClassifiedData<T>
where
    T: Deserialize<'de> + zeroize::Zeroize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = T::deserialize(deserializer)?;
        Ok(ClassifiedData::new(inner))
    }
}



impl Config {
    #[cfg(feature = "std")]
    pub fn load(path: &str) -> Result<Self, SecureError> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), SecureError> {
        // Implement comprehensive validation logic
        if !self.concurrency.is_valid() {
            return Err(SecureError::PipelineError("Invalid concurrency pattern".into()));
        }
        // Add more validation as needed
        Ok(())
    }
}

#[cfg(feature = "std")]
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config::load("config.toml").expect("Failed to load configuration")
});
#[cfg(feature = "std")]
pub fn get_config() -> &'static Config {
    &CONFIG
}
