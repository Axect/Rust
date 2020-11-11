extern crate strided;
extern crate peroxide;
use strided::{Stride, MutStride};
use peroxide::*;

fn main() {
    let mut a = zeros(10000, 10000);
    //let b = MutStride::new(&mut a.data[..]);
    //let mut substride = b.substrides_mut(a.row);
    //let mut v = substride.nth(999).unwrap();
    //for (i, e) in v.iter_mut().enumerate() {
    //    *e = i as f64;
    //}

    //let mut s = 0f64;
    //for e in v.iter_mut() {
    //    s += *e;
    //}
    //s.print();

    unsafe {
        let mut b = a.row_mut(999);
        for (i, &mut e) in b.iter_mut().enumerate() {
            *e = i as f64;
        }
        let mut s = 0f64;
        for e in b {
            s += *e;
        }
        s.print();
    }
}
