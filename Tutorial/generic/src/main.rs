use std::convert;
use std::fmt;
pub use self::Shape::{Row, Col};

fn main() {
    let p = Matrix::new(vec![1,2,3,4], 2, 2, Row);
    println!("{}", p);
    println!("{}", p.change_shape());
}

#[derive(Debug)]
pub enum Shape {
    Row,
    Col,
}

#[derive(Debug)]
pub struct Matrix {
    data: Vec<f64>,
    row: usize,
    col: usize,
    shape: Shape,
}

trait Array2D<T: convert::Into<f64>> {
    fn new(v: Vec<T>, x:usize, y:usize, shape: Shape) -> Matrix;
}

#[allow(dead_code)]
impl Matrix {
    fn change_shape(&self) -> Matrix {
        let r = self.row;
        let c = self.col;
        let l = r * c - 1;
        let mut data: Vec<f64> = self.data.clone();
        let ref_data: Vec<f64> = self.data.clone();

        match self.shape {
            Row => {
                for i in 0 .. l {
                    let s = (i * c) % l;
                    data[i] = ref_data[s];
                }
                data[l] = ref_data[l];
                Matrix {
                    data: data.clone(),
                    row: r,
                    col: c,
                    shape: Col,
                }
            },
            Col => {
                for i in 0 .. l {
                    let s = (i * r) % l;
                    data[i] = ref_data[s];
                }
                data[l] = ref_data[l];
                Matrix {
                    data: data.clone(),
                    row: r,
                    col: c,
                    shape: Row,
                }
            },
        }
    }

    fn spread(&self) -> String {
        //assert_eq!(self.row * self.col, self.data.len());
        //let rows = self.row;
        //let cols = self.col;
        //let data = self.data.clone();
        //
        //let mut result = String::new();

        //match self.shape {
        //    Row => {
        //        let temp: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();
        //        let ts: Vec<String> = temp.into_iter().take(rows).collect();
        //        let mut ss: Vec<String> = temp.into_iter().skip(rows).collect();
        //        result += &ts.join(",");
        //        while ss.len() >= rows {
        //            result += "\n";
        //            let ts: Vec<String> = ss.into_iter().take(rows).collect();
        //            result += &ts.join(",");
        //            ss = ss.into_iter().skip(rows).collect();
        //        }
        //    },
        //    Col => {
        //        let temp: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();
        //        let ts: 
        //    }
        //}
        //return result;
        unimplemented!();
    }
}

impl<T> Array2D<T> for Matrix where T: convert::Into<f64> {
    fn new(v: Vec<T>, x: usize, y: usize, shape: Shape) -> Matrix {
        Matrix {
            data: v.into_iter().map(|x| x.into()).collect(),
            row: x,
            col: y,
            shape: shape,
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix{:?}", self.data)
    }
}
