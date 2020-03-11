extern crate peroxide;
use peroxide::*;

fn main() {
    let a = rand(1000, 1000);
    let b = rand(1000, 1000);
    (a * b)[(99, 99)].print();
}
