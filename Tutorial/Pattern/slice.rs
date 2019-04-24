#![feature(slice_patterns)]

fn main() {
    let a: Vec<usize> = vec![1,2,3,4,5];
    println!("{}", reduce(a));
}

fn reduce(v: Vec<usize>) -> usize {
    match v.as_slice() {
        [x] => *x,
        [x, xs..] => *x + reduce(xs.to_vec()),
        _ => unreachable!()
    }
}
