extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;
//use std::env::args;

fn main() {
    //let args: Vec<String> = args().collect();
    //let num: usize = args[1].parse().unwrap();
    //if num == 1 {
    //    bench_1().print();
    //} else if num == 2 {
    //    bench_2().print();
    //} else {
    //    println!("err");
    //}
    // x = 3
    let x1 = vec![3f64, 1f64, 0f64, 0f64];
    x1.print();
    let x2 = mul(&x1, &x1);
    x2.print();
    let x3 = mul(&x2, &x1);
    x3.print();
    let x4 = mul(&x3, &x1);
    x4.print();
    let x5 = mul(&x3, &x2);
    x5.print();
    let x6 = mul(&x4, &x1);
    x6.print();

    let y1 = div(&x1, &x2);
    y1.print();
    let y2 = div(&x1, &x3);
    y2.print();

    let z1 = ln(&x1);
    z1.print();
    let z2 = ln(&x2);
    z2.print();

    let p1 = powf(&x1, 2f64);
    p1.print();
    let p2 = powf(&x1, 0.5f64);
    p2.print();
    let p3 = powf(&x1, 1.5f64);
    p3.print();
}
