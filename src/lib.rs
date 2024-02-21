use std::io;
use std::marker::PhantomData;
use mpi::traits::*;

/// Initialize the runtime.
pub fn new() -> Runtime {
    Runtime::new()
}

pub struct Runtime {
    universe: mpi::environment::Universe,
    world: mpi::topology::SimpleCommunicator,
    next_id: i32,
}

impl Runtime {
    /// Initialize the runtime.
    pub fn new() -> Runtime {
        let universe = mpi::initialize().unwrap();
        let world = universe.world();
        Runtime {
            universe,
            world,
            next_id: 0,
        }
    }

    pub fn read<T: Copy>(&self, fname: &str) -> io::Result<ParallelFile<T>> {
        Ok(ParallelFile {
            fname: fname.to_string(),
            phantom: PhantomData,
        })
    }
}

/// A parallel file that can be accessed and modified in a distributed fashion.
pub struct ParallelFile<T> {
    fname: String,
    phantom: PhantomData<T>,
}

impl<T: Copy> ParallelFile<T> {
    /// Map the values in the file in parallel using a closure.
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(T) -> T,
    {
        self
    }

    /// Write the parallel file out to fname.
    pub fn write(self, fname: &str) -> io::Result<Self> {
        Ok(self)
    }
}

/// Info about the current process.
struct ProcessInfo {
    size: i32,
    rank: i32,
}

pub trait DistributedIterator: Sized + Send {
    type Item: Send;

    /// Execute each `op` on each item in parallel (and distributed if running
    /// on a cluster).
    fn for_each<OP>(self, op: OP)
    where
        OP: Fn(Self::Item) + Sync + Send,
    {
        // TODO
    }
}
