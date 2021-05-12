use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let x = 5f64;
    let y = Rc::new(RefCell::new(x));

    let z = Rc::clone(&y);
    let w = Rc::clone(&y);

    *z.borrow_mut() += 1f64;
    *w.borrow_mut() += 2f64;

    println!("y: {}", y.borrow());
}
