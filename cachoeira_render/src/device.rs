use super::pipeline;
use std::sync::{Arc, RwLock};

struct Device {
}

trait UsesPipeline {
    fn set_pipeline(&mut self, pipeline: Arc<RwLock<pipeline::Pipeline>>) -> Result<bool, String>;
    
}

