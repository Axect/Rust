#![feature(const_generics)]

const n: usize = 1000000usize;

fn main() {
    let mut a: Foo<n> = Foo {
        data: [0f64; n]
    };
    for i in 0 .. n {
        a.data[i] = i as f64;
    }
    let b = a.data[n-1];
    println!("{}", b);
}

#[derive(Clone)]
struct Foo<const N: usize> {
    pub data: [f64; N],
}

