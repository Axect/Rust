use std::time;

fn main() {
    let now = time::Instant::now();

    let mut s = 0f64;
    for i in 0..10000_0000 {
        s += i as f64;
    }
    println!("{}", s);

    println!("{}", now.elapsed().as_secs_f64());
}
