use std::ops::{Index, IndexMut};

fn main() {
    let mut d = Data::new(1f64);
    let mut dr = Data::new(2f64);
    let mut dd = Data::new(3f64);
    let mut drd = Data::new(4f64);

    d.set_right(&mut dr);
    d.set_down(&mut dd);
    dr.set_down(&mut drd);
    dd.set_right(&mut drd);

    println!("{}, {}", d.value, dr.value);
    println!("{}, {}", dd.value, drd.value);

    unsafe {
        let mut curr_ptr = &mut d as *mut Data;
        print!("{}\t", (*curr_ptr).value);
        curr_ptr = d.right;
        print!("{}\n", (*curr_ptr).value);
        curr_ptr = d.down;
        print!("{}\t", (*curr_ptr).value);
        curr_ptr = (*curr_ptr).right;
        print!("{}\n", (*curr_ptr).value);
    }

    println!("d[1,1] = {}", d[(1,1)]);

    d[(1,1)] = 2f64;

    println!("d[1,1] = {}", d[(1,1)]);

    let mut e = Data::empty();
    e.from_vec(&vec![1f64, 2f64, 3f64]);
    println!("e[0,2] = {}", e[(0,2)]);
}

struct Data {
    value: f64,
    right: *mut Data,
    down: *mut Data,
}

impl Data {
    fn empty() -> Self {
        Data {
            value: 0f64,
            right: std::ptr::null_mut(),
            down: std::ptr::null_mut(),
        }
    }

    fn new(value: f64) -> Self {
        Data {
            value,
            right: std::ptr::null_mut(),
            down: std::ptr::null_mut(),
        }
    }

    fn set_right(&mut self, source: *mut Data) {
        self.right = source;
    }

    fn set_down(&mut self, source: *mut Data) {
        self.down = source;
    }

    fn from_vec<'a>(&mut self, v: &'a Vec<f64>) {
        self.value = v[0];
        
        unsafe {
            let mut curr_ptr = self as *mut Self;
            for i in 1 .. v.len() {
                (*curr_ptr).right = &mut Data::new(v[i]) as *mut Self;
                curr_ptr = (*curr_ptr).right
            }
        }
    }
}

impl Index<(usize, usize)> for Data {
    type Output = f64;
    fn index(&self, idx: (usize, usize)) -> &f64 {
        unsafe {
            let mut curr_ptr = self as *const Self;
            for i in 0 .. idx.1 {
                curr_ptr = (*curr_ptr).right;
            }
            for j in 0 .. idx.0 {
                curr_ptr = (*curr_ptr).down;
            }
            &(*curr_ptr).value
        }
    }
}

impl IndexMut<(usize, usize)> for Data {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut f64 {
        unsafe {
            let mut curr_ptr = self as *mut Self;
            for i in 0 .. idx.0 {
                curr_ptr = (*curr_ptr).right;
            }
            for j in 0 .. idx.1 {
                curr_ptr = (*curr_ptr).down;
            }
            &mut (*curr_ptr).value
        }
    }
}
