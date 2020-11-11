extern crate cblas;
use cblas::*;

fn main() {
    let (m, n, k) = (2, 4, 3);
    let a = vec![
        1.0, 4.0,
        2.0, 5.0,
        3.0, 6.0,
    ];
    let b = vec![
        1.0, 5.0,  9.0,
        2.0, 6.0, 10.0,
        3.0, 7.0, 11.0,
        4.0, 8.0, 12.0,
    ];
    let mut c = vec![
        2.0, 7.0,
        6.0, 2.0,
        0.0, 7.0,
        4.0, 2.0,
    ];

    unsafe {
        dgemm(Layout::ColumnMajor, Transpose::None, Transpose::None, m, n, k, 1.0, &a, m, &b, k, 1.0, &mut c, m);
    }

    println!("{:?}", c);
}
