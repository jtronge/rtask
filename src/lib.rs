//! Rust distributed-parallel programming library.
use std::sync::Arc;

mod comm;
use comm::CommunicationRuntime;
mod iter;

pub mod prelude {
    pub use crate::iter::{DistributedIterator, IntoDistributedIterator};
}

pub struct Runtime {
    comm_runtime: Arc<CommunicationRuntime>,
}

/// Initialize the runtime.
pub fn new() -> Runtime {
    Runtime {
        comm_runtime: CommunicationRuntime::new(),
    }
}
