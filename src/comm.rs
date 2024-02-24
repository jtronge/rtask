//! Communication library for parallel computing.
//!
//! This is designed to abstract away the underlying communication library
//! being used.
use std::sync::Arc;
use mpi::traits::*;
use mpi::environment::Universe;
use mpi::topology::SimpleCommunicator;

pub(crate) struct CommunicationRuntime {
    _universe: Universe,
    world: SimpleCommunicator,
    size: i32,
    rank: i32,
}

impl CommunicationRuntime {
    pub(crate) fn new() -> Arc<CommunicationRuntime> {
        let universe = mpi::initialize().unwrap();
        let world = universe.world();
        let size = world.size();
        let rank = world.rank();
        Arc::new(CommunicationRuntime {
            _universe: universe,
            world,
            size,
            rank,
        })
    }

    /// Return the total number of processes running.
    #[inline]
    pub(crate) fn count(&self) -> i32 {
        self.size
    }

    /// Return the ID of this process.
    #[inline]
    pub(crate) fn id(&self) -> i32 {
        self.rank
    }
}
