use rtask;
use std::io;

fn main() -> io::Result<()> {
    let mut runtime = rtask::new();

    runtime.read::<u8>("data/test-0.dat")?
        .map(|x| x * 2)
        .write("tmp/test-0-out.dat")?;

    Ok(())
}
