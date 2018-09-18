#[macro_use(array)]
extern crate ndarray;

#[macro_use]
extern crate rulinalg;

mod ndarray_test;
mod rulinalg_test;

fn main() {
    ndarray_test::nd();
    rulinalg_test::ru();
}