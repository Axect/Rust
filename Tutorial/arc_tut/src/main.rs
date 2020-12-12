use std::thread;
use std::time::Duration;
use std::sync::Arc;

fn main() {
    // Compile Error
    //let foo = vec![0];
    //thread::spawn(|| {
    //    thread::sleep(Duration::from_millis(20));
    //    println!("{:?}", &foo);
    //});
    
    let foo = Arc::new(vec![0]);
    let bar = Arc::clone(&foo);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        println!("{:?}", *bar);
    });

    println!("{:?}", foo);
}
