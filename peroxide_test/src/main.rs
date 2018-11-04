extern crate peroxide;

use peroxide::*;

fn main() {
    let s = seq!(0;1;0.01);
    let a = matrix(s.clone(), s.len(), 1, Col);
    let b = a.fmap(|x| x.powf(2.));
    let c = cbind!(a, b);
    c.write("dat.csv");
}
