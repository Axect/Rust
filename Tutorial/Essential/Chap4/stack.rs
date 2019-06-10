fn main() {
    let mut v = [0f64; 10_000_000];

    for i in 0 .. v.len() {
        v[i] = i as f64;
    }

    let mut s = 0f64;

    for a in v.iter() {
        s += a;
    }

    println!("{}", s);
}
