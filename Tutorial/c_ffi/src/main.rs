extern crate libc;

extern {
    fn doubler(x: libc::c_int) -> libc::c_int;
}

fn main() {
    unsafe {
        println!("{}", doubler(1));
    }
}
