extern crate peroxide;
use peroxide::*;
use std::ops::{Add, Sub};

fn main() {
    let mut a = zeros(5, 5);
    a[(0, 2)] = 2f64;
    a[(3, 1)] = -3f64;
    a[(4, 4)] = 1f64;

    let mut b = zeros(5, 5);
    b[(0, 1)] = 3f64;
    b[(2, 2)] = -2f64;
    b[(3, 1)] = 4f64;
    b[(4, 3)] = 2f64;
    
    let m = SPRS::from_dense(&a);
    let n = SPRS::from_dense(&b);
    println!("{:?}", m.clone() + n.clone());
    println!("{:?}", m.clone() - n.clone());
    (&a + &b).print();
    (&a - &b).print();
}

#[derive(Debug, Clone)]
struct SPRS {
    row: usize,
    col: usize,
    tot: usize,
    data: Vec<((usize, usize), f64)>,
}

impl SPRS {
    fn from_dense(m: &Matrix) -> Self {
        let row = m.row;
        let col = m.col;
        let mut data: Vec<((usize, usize), f64)> = Vec::new();
        
        for i in 0 .. row {
            for j in 0 .. col {
                let val = m[(i,j)];
                if val != 0f64 {
                    data.push(((i, j), val));
                } 
            }
        }
        
        SPRS {
            row,
            col,
            tot: data.len(),
            data: data,
        }
    }
}

impl Add<SPRS> for SPRS {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let row = self.row;
        let col = self.col;
        let mut i = 0usize;
        let mut j = 0usize;

        let mut data: Vec<((usize, usize), f64)> = Vec::new();

        while i < self.tot && j < other.tot {
            let cmp = compare(&self.data[i].0, &other.data[j].0);
            if cmp == Compare::Small {
                data.push(self.data[i]);
                i += 1;
            } else if cmp == Compare::Large {
                data.push(other.data[j]);
                j += 1;
            } else {
                let (idx, val1) = self.data[i];
                let (_, val2) = other.data[j];
                data.push((idx, val1 + val2));
                i += 1;
                j += 1;
            }
        }

        while i < self.tot {
            data.push(self.data[i]);
            i += 1;
        }

        while j < other.tot {
            data.push(self.data[j]);
            j += 1;
        }

        SPRS {
            row,
            col,
            tot: data.len(),
            data,
        }
    }
}

impl Sub<SPRS> for SPRS {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let row = self.row;
        let col = self.col;
        let mut i = 0usize;
        let mut j = 0usize;

        let mut data: Vec<((usize, usize), f64)> = Vec::new();

        while i < self.tot && j < other.tot {
            let cmp = compare(&self.data[i].0, &other.data[j].0);
            if cmp == Compare::Small {
                data.push(self.data[i]);
                i += 1;
            } else if cmp == Compare::Large {
                let (idx, val) = other.data[j];
                data.push((idx, -val));
                j += 1;
            } else {
                let (idx, val1) = self.data[i];
                let (_, val2) = other.data[j];
                data.push((idx, val1 - val2));
                i += 1;
                j += 1;
            }
        }

        while i < self.tot {
            data.push(self.data[i]);
            i += 1;
        }

        while j < other.tot {
            data.push(self.data[j]);
            j += 1;
        }

        SPRS {
            row,
            col,
            tot: data.len(),
            data,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Compare {
    Large,
    Equal,
    Small,
}

fn compare(a: &(usize, usize), b: &(usize, usize)) -> Compare {
    if a.0 != b.0 {
        if a.0 > b.0 {
            Compare::Large
        } else {
            Compare::Small
        }
    } else {
        if a.1 > b.1 {
            Compare::Large
        } else if a.1 < b.1 {
            Compare::Small
        } else {
            Compare::Equal
        }
    }
}
