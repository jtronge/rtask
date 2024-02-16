use rtask;

fn main() {
    let mut runtime = rtask::new();

    let mut tasks = vec![];
    for i in 0..16 {
        tasks.push(runtime.spawn(move || {
            println!("hello from task {}", i);
        }));
    }

    runtime.waitall(tasks);
}
