use std::ops::Index;

fn main() {
    let mut d = Data::new(1f64);
    let mut dr = Data::new(2f64);
    let mut dd = Data::new(3f64);
    let mut drd = Data::new(4f64);

    d.set_right(&dr);
    d.set_down(&dd);
    dr.set_down(&drd);
    dd.set_right(&drd);

    println!("{}, {}", d.value, dr.value);
    println!("{}, {}", dd.value, drd.value);

    unsafe {
        let mut curr_ptr = &d as *const Data;
        print!("{}\t", (*curr_ptr).value);
        curr_ptr = d.right;
        print!("{}\n", (*curr_ptr).value);
        curr_ptr = d.down;
        print!("{}\t", (*curr_ptr).value);
        curr_ptr = (*curr_ptr).right;
        print!("{}\n", (*curr_ptr).value);
    }

    println!("d[1,1] = {}", d[(1,1)]);
}

struct Data {
    value: f64,
    right: *const Data,
    down: *const Data,
}

impl Data {
    fn new(value: f64) -> Self {
        Data {
            value,
            right: std::ptr::null(),
            down: std::ptr::null(),
        }
    }

    fn set_right(&mut self, source: *const Data) {
        self.right = source;
    }

    fn set_down(&mut self, source: *const Data) {
        self.down = source;
    }
}

impl Index<(usize, usize)> for Data {
    type Output = f64;
    fn index(&self, idx: (usize, usize)) -> &f64 {
        unsafe {
            let mut curr_ptr = self as *const Self;
            for i in 0 .. idx.0 {
                curr_ptr = (*curr_ptr).right;
            }
            for j in 0 .. idx.1 {
                curr_ptr = (*curr_ptr).down;
            }
            &(*curr_ptr).value
        }
    }
}
