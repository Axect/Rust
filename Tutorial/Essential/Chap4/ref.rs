fn main() {
    let n = 10_000_000usize;
    let mut v = vec![0f64; n];
    let mut s = 0f64;

    for i in 0 .. v.len() {
        v[i] = i as f64;
    }

    for b in &v {
        s += *b;
    }

    println!("{}", s);
}
