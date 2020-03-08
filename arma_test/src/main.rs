#[macro_use]
extern crate cpp;

cpp!{{
    #include <iostream>
    #include <armadillo>

    using namespace arma;
}}

fn main() {
    let row = 5usize;
    let col = 5usize;
    // Seed
    unsafe {
        cpp!([] -> () as "void" {
            arma::arma_rng::set_seed_random();
        });
    }
    let m = unsafe {
        cpp!([row as "uword", col as "uword"] -> f64 as "double_t" {
            mat m = randu<mat>(row, col);
            return det(m);
        })
    };
    println!("{}", m);
}
