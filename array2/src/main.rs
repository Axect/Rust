extern crate time;

fn main() {
    // into_boxed_slice is dynamical fixed array
    // Only array can make below 10^6

    const NUM: usize = 100_000_000;

    let start = time::now();
    let mut v : Vec<i64> = std::iter::repeat(0).take(NUM).collect();

    for i in 0..NUM {
        v[i] = (i as i64) + 1;
    }
    let w = &v;

    let mut s: i64 = 0;
    for i in 0..NUM {
        s += w[i]
    }
    let end = time::now() - start;
    println!("{}",s);
    println!("{}", end);
}