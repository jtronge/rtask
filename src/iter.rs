//! Distributed iterator traits and helper code.
use std::convert::From;
use std::iter::{Iterator, Map};
use std::ops::Range;
use std::sync::Arc;
use crate::Runtime;
use crate::comm::CommunicationRuntime;

/// The distributed iterator trait provides methods for combining distributed
/// iterator computations across multiple processes.
pub trait DistributedIterator: Iterator {
    /// Collect all values from all processes and return them in a vector.
    fn collect_all(self) -> Vec<<Self as Iterator>::Item>;
}

/// Trait for converting from some single-process iterator into a distributed
/// version.
pub trait IntoDistributedIterator {
    type Item;
    type Result: DistributedIterator + Iterator<Item=Self::Item>;

    /// Return a type that implements DistributedIterator.
    fn dist_iter(self, runtime: &Runtime) -> Self::Result;
}

impl<B, I, F> DistributedIterator for Map<I, F>
where
    I: Iterator + DistributedIterator,
    F: FnMut(I::Item) -> B,
{
    fn collect_all(self) -> Vec<<Self as Iterator>::Item> {
        // Local collect()
        let data = self.collect();
        // Communication
        data
    }
}

pub trait RangeIdx: num::Integer + Copy + From<i32> + std::fmt::Debug{}
impl<T: num::Integer + Copy + From<i32> + std::fmt::Debug> RangeIdx for T {}

/// A range that can be possibly distributed over multiple processes, nodes,
/// etc.
pub struct DistributedRange<Idx> {
    range: Range<Idx>,
    local_range: Range<Idx>,
    comm_runtime: Arc<CommunicationRuntime>,
}

impl<Idx: RangeIdx> Iterator for DistributedRange<Idx> {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        // For some reason this now requires a Step trait that is experimental
        // self.local_range.next()
        let next = self.local_range.start;
        if next >= self.local_range.end {
            None
        } else {
            self.local_range.start = next + 1.into();
            Some(next)
        }
    }
}

impl<Idx: RangeIdx> DistributedIterator for DistributedRange<Idx> {
    fn collect_all(self) -> Vec<<Self as Iterator>::Item> {
        vec![]
    }
}

impl<Idx: RangeIdx> IntoDistributedIterator for Range<Idx> {
    type Item = Idx;
    type Result = DistributedRange<Idx>;

    fn dist_iter(self, runtime: &Runtime) -> Self::Result {
        let count = runtime.comm_runtime.count();
        let id = runtime.comm_runtime.id();

        // Compute the local range to iterate over
        let id: Idx = id.try_into().unwrap();
        let count: Idx = count.try_into().unwrap();
        let total: Idx = self.end - self.start;
        let local_total = total / count;
        let local_total = if local_total == 0.into() { 1.into() } else { local_total };
        let local_start = id * local_total;
        let local_range = if local_start >= self.end {
            Range {
                start: self.end,
                end: self.end,
            }
        } else {
            let local_end = local_start + local_total;
            Range {
                start: local_start,
                end: if local_end > self.end { self.end } else { local_end },
            }
        };

        DistributedRange {
            range: self,
            local_range,
            comm_runtime: Arc::clone(&runtime.comm_runtime),
        }
    }
}
