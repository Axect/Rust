use std::ops::Deref;

fn main() {
    let mut a = Matrix::new(vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64], 2, 3, Shape::Row);
    let mut p = a.row_ptr();

    unsafe {
        p.map(|x| *x).for_each(|t| println!("{}", t));
    }
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    Row,
    Col,
}

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<f64>,
    row: usize,
    col: usize,
    shape: Shape,
}

impl Matrix {
    fn new(data: Vec<f64>, row: usize, col: usize, shape: Shape) -> Self {
        Matrix {
            data,
            row,
            col,
            shape,
        }
    }

    fn row_ptr(&mut self) -> SizedPtr {
        SizedPtr {
            ptr: &mut self.data[0] as *mut f64,
            size: self.col,
            limit: self.row,
        }
    }

    fn col_ptr(&mut self) -> SizedPtr {
        SizedPtr {
            ptr: &mut self.data[0] as *mut f64,
            size: self.row,
            limit: self.row,
        }
    }
}

struct SizedPtr {
    ptr: *mut f64,
    size: usize,
    limit: usize,
}

impl Deref for SizedPtr {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl Iterator for SizedPtr {
    type Item = *mut f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.limit == 0 {
            return None;
        }

        let p = self.ptr;

        self.limit -= 1;
        unsafe {
            self.ptr = self.ptr.add(self.size);
            Some(p)
        }
    }
}
