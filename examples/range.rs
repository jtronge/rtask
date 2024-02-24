use rtask;
use rtask::prelude::*;

fn main() {
    let mut runtime = rtask::new();

    let data: Vec<i64> = (0..1024*1024)
        .dist_iter(&runtime)
        .map(|i| i + 1)
        .collect_all();

    assert_eq!(1024*1024, data.len());
    for (i, value) in data.iter().enumerate() {
        let expected: i64 = (i + 1).try_into().unwrap();
        assert_eq!(expected, *value);
    }
}
