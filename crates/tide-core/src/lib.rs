pub mod stream;
pub mod types;
pub mod validator;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crossbeam_channel::{Receiver, Sender};
use anyhow::Result;

pub struct TideEngine {
    running: Arc<AtomicBool>,
    producer: Sender<types::TideData>,
    consumer: Receiver<types::TideData>,
}

impl TideEngine {
    /// Create a new Tide engine instance
    pub fn new() -> Result<Self> {
        let (producer, consumer) = crossbeam_channel::bounded(4096);
        
        Ok(TideEngine {
            running: Arc::new(AtomicBool::new(false)),
            producer,
            consumer,
        })
    }

    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting Tide engine");
        self.running.store(true, Ordering::SeqCst);
        
        let validator = validator::ValidatorIntegration::new()?;
        
        self.run_pipeline(validator).await
    }

    pub fn stop(&self) {
        tracing::info!("Stopping Tide engine");
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn get_receiver(&self) -> Receiver<types::TideData> {
        self.consumer.clone()
    }

    async fn run_pipeline(&self, validator: validator::ValidatorIntegration) -> Result<()> {
        let pipeline = stream::Pipeline::new(validator, self.producer.clone());
        
        while self.running.load(Ordering::SeqCst) {
            pipeline.process_batch().await?;
        }
        
        Ok(())
    }
}