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

    /// Spawn a new task.
    pub fn spawn<F>(&mut self, f: F) -> i32
    where
        F: FnOnce() + 'static,
    {
        // Just evenly distribute tasks
        let rank = self.world.rank();
        let size = self.world.size();
        let id = self.next_id;
        self.next_id += 1;
        if (id % size) == rank {
            println!("running {} on MPI rank {}", id, rank);
            f();
        }
        id
    }

    pub fn waitall(&self, tasks: Vec<i32>) {
    }
}
