use rtask;
use rtask::prelude::*;

fn main() {
    let mut runtime = rtask::new();

    let data: Vec<u64> = (0..1024*1024)
        .dist_iter(&runtime)
        .map(|i| i + 1)
        .collect_all();

    for (i, value) in data.iter().enumerate() {
        let expected: u64 = (i + 1).try_into().unwrap();
        assert_eq!(expected, *value);
    }
}
