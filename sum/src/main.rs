fn main() {
    // If not s = i64 then overflow (integer != i64)
    let mut s = 0 as i64;
    for i in 1..100001 {
        s += i;
    }
    println!("{}", s)
}