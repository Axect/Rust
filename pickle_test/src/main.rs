extern crate serde;
extern crate serde_pickle;
extern crate peroxide;

use peroxide::*;
use serde_pickle as pickle;
use std::fs::File;
use std::io::{Write};

fn main() {
    let a = Matrix::from_index(|i, j| i + j, (5, 5));
    a.print();

    let mut writer: Box<Write> = Box::new(File::create("hello").unwrap());

    pickle::to_writer(&mut writer, &vec![a.row(0), a.row(1)], true).expect("pickle error");
}
