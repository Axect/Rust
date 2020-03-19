fn main() {
    let mut foo = Foo::with_len(100_0000);
    for i in 0 .. foo.data.len() {
        foo.data[i] = i as f64;
    }
    println!("{}", foo.data[99_9999]);
}

#[derive(Debug, Clone)]
struct Foo {
    pub data: Vec<f64>,
}

impl Foo {
    fn with_len(n: usize) -> Self {
        Self {
            data: vec![0f64; n],
        }
    }

    fn from_vec(v: &Vec<f64>) -> Self {
        Self {
            data: v[..].to_vec(),
        }
    }
}
