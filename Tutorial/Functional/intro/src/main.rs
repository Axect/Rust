#![feature(extern_prelude)]

mod intro_generic;
mod intro_function;
mod intro_iterator;
mod intro_expression;

use intro_generic::*;
use intro_function::f;
use intro_iterator::iter1;
use intro_expression::term;

fn main() {
    println!("{}", foo(2));
    println!("{}", bar(foo, 2));
    println!("{}", f(|x|{ x*x }, 2));
    println!("{}", iter1());
    println!("{}", term());
}
