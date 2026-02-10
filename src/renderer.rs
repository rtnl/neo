use crate::program::Program;
use std::sync::Arc;

pub struct Renderer {}

impl Renderer {
    pub async fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub(crate) async fn launch(&self, program: Arc<Program>) {
        tokio::spawn(async move {});
    }
}
