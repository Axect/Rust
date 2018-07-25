#![feature(extern_prelude)]

mod intro_generic;
mod intro_function;

use intro_generic::*;
use intro_function::f;

fn main() {
    println!("{}", foo(2));
    println!("{}", bar(foo, 2));
    println!("{}", f(foo, 2));
}
