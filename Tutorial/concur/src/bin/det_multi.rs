extern crate crossbeam;
extern crate peroxide;
use peroxide::*;

pub fn main() {
    multi_det(10).unwrap().print();
}

fn multi_det(num: usize) -> Option<f64> {
    let mut sum = 0f64;
    crossbeam::scope(|s| {
        for _i in 0 .. num {
            let thread = s.spawn(|_| rand_det());
            sum += thread.join().unwrap();
        }
        Some(sum)
    }).unwrap()
}

fn rand_det() -> f64 {
    rand(50, 50).det()
}
