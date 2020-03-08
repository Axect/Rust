extern crate sparse_ccs;
extern crate peroxide;
use peroxide::*;
use sparse_ccs::*;

fn main() {
    let a = ml_matrix("
        3 0 1 2 0;
        0 4 0 0 0;
        0 7 5 9 0;
        0 0 0 0 0;
        0 0 0 6 5
    ");
    
    a.print();

    let b = SPMatrix::from_dense(&a);
    println!("{:?}", b);

    b.to_dense().print();

    let c = c!(1, 2, 3, 4, 5);
    
    let result = &b * &c;
    let compar = &a * &c;

    result.print();
    compar.print();
}
