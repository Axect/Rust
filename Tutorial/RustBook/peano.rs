#[derive(Debug)]
enum Peano {
    Zero,
    Succ(Box<Peano>)
}

use Peano::Zero;
use Peano::Succ;

impl PartialEq for Peano {
    fn eq(&self, other:&Peano) -> bool {
        match (self, other) {
            (&Zero, &Zero) => true,
            (&Succ(ref a), &Succ(ref b)) => (a==b),
            (_,_) => false
        }
    }
}

fn main() {
    println!("{}", Zero==Zero);
}