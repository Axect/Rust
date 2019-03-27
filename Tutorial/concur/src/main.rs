use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        println!("Hello from the goblin in the spawned thread!");
    });
    let output = handle.join().unwrap();
    println!("{:?}", output);
}
