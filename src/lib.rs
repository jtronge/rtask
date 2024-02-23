//! Rust distributed-parallel programming library.
use std::iter::{Iterator, Map};
use std::ops::Range;

pub struct Runtime;

/// Initialize the runtime.
pub fn new() -> Runtime {
    Runtime
}

/// Private info about the current running process.
struct ProcessInfo {
    size: i32,
    rank: i32,
}

/// The distributed iterator trait provides methods for combining distributed
/// iterator computations across multiple processes.
pub trait DistributedIterator: Iterator {
    fn collect_all(&self) -> Vec<<Self as Iterator>::Item>;
}

/// A range that can be possibly distributed over multiple processes, nodes,
/// etc.
pub struct DistributedRange<Idx> {
    range: Range<Idx>,
    process_info: ProcessInfo,
}

impl<Idx> Iterator for DistributedRange<Idx> {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<Idx> DistributedIterator for DistributedRange<Idx> {
    fn collect_all(&self) -> Vec<<Self as Iterator>::Item> {
        vec![]
    }
}

impl<B, I, F> DistributedIterator for Map<I, F>
where
    I: Iterator + DistributedIterator,
    F: FnMut(I::Item) -> B,
{
    fn collect_all(&self) -> Vec<<Self as Iterator>::Item> {
        vec![]
    }
}

/// Trait for converting from some single-process iterator into a distributed
/// version.
pub trait IntoDistributedIterator {
    type Item;
    type Result: DistributedIterator + Iterator<Item=Self::Item>;

    fn dist_iter(self, runtime: &Runtime) -> Self::Result;
}

impl<Idx> IntoDistributedIterator for Range<Idx> {
    type Item = Idx;
    type Result = DistributedRange<Idx>;

    fn dist_iter(self, runtime: &Runtime) -> Self::Result {
        DistributedRange {
            range: self,
            process_info: ProcessInfo {
                size: 0,
                rank: 0,
            },
        }
    }
}

pub mod prelude {
    pub use super::{DistributedIterator, IntoDistributedIterator};
}
