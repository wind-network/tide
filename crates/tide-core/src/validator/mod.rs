pub mod mmap;
pub mod pipeline;
pub mod simd;

use anyhow::Result;
use memmap2::MmapMut;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct ValidatorIntegration {
    tpu_buffer: Arc<MmapMut>,
    tvu_buffer: Arc<MmapMut>,
    latest_tpu: AtomicPtr<u8>,
    latest_tvu: AtomicPtr<u8>,
}

impl ValidatorIntegration {
    pub fn new() -> Result<Self> {
        let tpu_buffer = mmap::create_shared_buffer(mmap::BufferSize::Large)?;
        let tvu_buffer = mmap::create_shared_buffer(mmap::BufferSize::Medium)?;
        
        Ok(ValidatorIntegration {
            latest_tpu: AtomicPtr::new(std::ptr::null_mut()),
            latest_tvu: AtomicPtr::new(std::ptr::null_mut()),
            tpu_buffer: Arc::new(tpu_buffer),
            tvu_buffer: Arc::new(tvu_buffer),
        })
    }

    pub fn attach_tpu(&self) -> Result<()> {
        pipeline::attach_pipeline(pipeline::PipelineStage::TPU, self.tpu_buffer.clone())
    }

    pub fn attach_tvu(&self) -> Result<()> {
        pipeline::attach_pipeline(pipeline::PipelineStage::TVU, self.tvu_buffer.clone())
    }

    pub fn read_tpu_data(&self) -> Option<&[u8]> {
        let ptr = self.latest_tpu.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }
        unsafe {
            Some(std::slice::from_raw_parts(ptr, mmap::BufferSize::Large as usize))
        }
    }

    pub fn process_with_simd(&self, data: &[u8]) -> Result<Vec<u8>> {
        simd::process_validator_data(data)
    }
}