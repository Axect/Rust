extern crate libc;
use libc::c_int;

#[link(name = "sum2")]
extern {
    fn sum2(x: c_int) -> c_int;
}

fn main() {
    unsafe {
        println!("{}", sum2(1));
    }
}
