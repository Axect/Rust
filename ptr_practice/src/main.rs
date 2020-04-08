use std::mem;
use std::ptr::{null, null_mut};

fn main() {
    let mut a: TVec<usize> = TVec::new();
    let mut b: [usize; 3] = [1,2,3];
    a.ptr = b.as_mut_ptr();
    a.cap = 3;
    a.len = 3;
    unsafe {
        println!("{}", *a.ptr);
        println!("{}", *a.ptr.offset(1));
        println!("{}", *a.ptr.offset(2));
    }
}

struct TVec<T> {
    pub ptr: *mut T,
    pub cap: usize,
    pub len: usize,
}

impl<T> TVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0);
        TVec {
            ptr: null_mut::<T>(),
            cap: 0,
            len: 0,
        }
    }
}