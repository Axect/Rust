use peroxide::fuga::*;

fn main() {
    let r = rand(100, 1).col(0).fmap(|t| t * 100f64);
    let u = vec![0usize, 1, 2, 3, 4];
    for x in r {
        fmt_f64(x, u.sample(1)[0]).print();
    }
}

/// Format f64 into lower exponent notation with '+' sign
/// * Example: 132.45 -> 1.32e+2
/// * Example: 0.1324 -> 1.32e-2
pub fn fmt_f64(f: f64, precision: usize) -> String {
    // s is lower exponent notation
    // ex) s = "1.32e-2", s = "1.32e2"
    // We should change the latter case to "1.32e+2"
    let mut s = format!("{:.p$e}", f, p=precision);
    let s_old = s.clone();
    let mut e = s.split_off(s.find('e').unwrap());
    if e.starts_with("e-") {
        s_old
    } else {
        e.insert(1, '+');
        format!("{}{}", s, e)
    }
}
