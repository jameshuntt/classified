//! ----------------------------------------------
//! FEATURE NOTES --------------------------------
//! ----------------------------------------------
//! feature_name:async
//! deps:[tokio][async_trait]
//! scope:[]
//! effected_lines:[]
//! corpus:true
//! ----------------------------------------------
//! feature_name:logging
//! deps:[tracing]
//! scope:[]
//! effected_lines:[]
//! corpus:false
//! ----------------------------------------------
//! feature_name:std
//! deps:[std]
//! scope:[]
//! effected_lines:[]
//! corpus:false
//! ----------------------------------------------
//! 
//! 
//! ----------------------------------------------
//! CORPUS FEATURES ------------------------------
//! ----------------------------------------------
#![cfg(feature = "async")]
#![cfg(feature = "std")]
//! ----------------------------------------------
//! 
//! 
//! 
//! 
//! filename:
//! 
//! 
//! usages:
//! 
//! 
//! 
//! 

use tokio::task;

pub struct ThreadPoolManager;

impl ThreadPoolManager {
    pub fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        task::spawn(future);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn spawns_and_executes_task() {
        let manager = ThreadPoolManager;
        let flag = Arc::new(Mutex::new(false));

        let flag_clone = Arc::clone(&flag);
        manager.spawn(async move {
            *flag_clone.lock().unwrap() = true;
        });

        sleep(Duration::from_millis(50)).await; // give time for task to run
        assert_eq!(*flag.lock().unwrap(), true);
    }

    #[tokio::test]
    async fn handles_multiple_tasks() {
        let manager = ThreadPoolManager;
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            manager.spawn(async move {
                let mut lock = counter_clone.lock().unwrap();
                *lock += 1;
            });
        }

        sleep(Duration::from_millis(100)).await;
        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[tokio::test]
    async fn tasks_can_be_async() {
        let manager = ThreadPoolManager;
        let result = Arc::new(Mutex::new(0));

        let result_clone = Arc::clone(&result);
        manager.spawn(async move {
            sleep(Duration::from_millis(10)).await;
            *result_clone.lock().unwrap() = 42;
        });

        sleep(Duration::from_millis(50)).await;
        assert_eq!(*result.lock().unwrap(), 42);
    }
}
