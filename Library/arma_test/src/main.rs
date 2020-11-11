// #[macro_use]
// extern crate cpp;
extern crate peroxide;
use peroxide::*;
use std::os::raw::{c_double, c_ulong};

// =============================================================================
// CPP Crate
// =============================================================================

// cpp!{{
//     #include <iostream>
//     #include <armadillo>

//     using namespace arma;
// }}

fn main() {
    // let row = 5usize;
    // let col = 5usize;
    // // Seed
    // unsafe {
    //     cpp!([] -> () as "void" {
    //         arma::arma_rng::set_seed_random();
    //     });
    // }
    // let m = unsafe {
    //     cpp!([row as "uword", col as "uword"] -> f64 as "double_t" {
    //         mat m = randu<mat>(row, col);
    //         return det(m);
    //     })
    // };
    // println!("{}", m);

    let n = rand(4, 4);
    let a = ARMA_Wrapper::from_matrix(&n);
    a.det().print()
}

// =============================================================================
// Using Only CC
// =============================================================================

#[repr(C)]
pub struct ARMA_Wrapper {
    pub data: *const c_double,
    pub row: c_ulong,
    pub col: c_ulong,
}

#[link(name = "arma")]
extern "C" {
    fn det(m: *const ARMA_Wrapper) -> f64;
}

impl ARMA_Wrapper {
    fn from_matrix(m: &Matrix) -> Self {
        Self {
            data: m.data.as_ptr(),
            row: m.row as u64,
            col: m.col as u64,
        }
    }

    fn det(&self) -> f64 {
        unsafe {
            det(self)
        }
    }
}
