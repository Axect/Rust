extern crate peroxide;
use peroxide::fuga::*;
use std::cell::RefCell;

fn main() {
}

#[derive(Clone, Copy)]
pub struct Var<'t> {
    tape: &'t Tape,
    index: usize,
    value: f64,
}

pub struct Tape { nodes: RefCell<Vec<Node>> }

pub struct Node {
    weights: [f64; 2],
    deps: [usize; 2],
}

impl<'t> Var<'t> {
    pub fn sin(self) -> Self {
        Var {
            tape: self.tape,
            value: self.value.sin(),
            index: self.tape.push1(
                self.index, self.value.cos(),
            ),
        }
    }
}

impl Tape {
    fn push1(&self, dep0: usize, weight0: f64) -> usize {
        let mut nodes = self.nodes.borrow_mut();
        let len = nodes.len();
        nodes.push(
            Node {
                weights: [weight0, 0.0],
                deps: [dep0, len],
            }
        );
        len
    }
}
