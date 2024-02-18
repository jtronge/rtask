//! Example of rayon's parallel iterators.
use rayon::prelude::*;

fn main() {
    let x: Vec<i32> = (0..128).collect();
    let total: i32 = x.par_iter()
        .map(|&val| val * 2)
        .sum();
    println!("total: {}", total);
}
