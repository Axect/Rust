extern crate time;

fn main() {
    // into_boxed_slice is dynamical fixed array
    // Only array can make below 10^6
    let start = time::now();
    let mut a = vec![0;100000000].into_boxed_slice();
    for i in 0..a.len() {
        a[i] = (i as i64) + 1;
    }
    let b = &a;

    let mut s: i64 = 0;
    for i in 0..b.len() {
        s += b[i];
    }
    let end = time::now() - start;
    println!("{}",s);
    println!("{}", end);
}