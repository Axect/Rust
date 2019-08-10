fn main() {
    let t1 = Test::new(1);
    let t2 = Test::wrap(2, &t1);

    println!("t2: {}", t2.value);
    unsafe {
        println!("t1: {}", (*t2.point).value);
    }
}

struct Test {
    value: usize,
    point: *const Test,
}

impl Test {
    fn new(value: usize) -> Self {
        Test { value, point: std::ptr::null() }
    }

    fn wrap(value: usize, target: *const Test) -> Self {
        Test {
            value,
            point: target
        }
    }
}
