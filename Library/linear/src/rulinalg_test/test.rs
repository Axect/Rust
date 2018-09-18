use crate::rulinalg::matrix::Matrix;

pub fn test() {
    let a = Matrix::new(2, 2, vec![
        1.0, 2.0,
        3.0, 4.0,
    ]);
    
    let b = matrix![
        1.0, 2.0, 3.0;
        4.0, 5.0, 6.0];

    println!("{}, {}", a, b);
    println!("{}", &a * &b);
}