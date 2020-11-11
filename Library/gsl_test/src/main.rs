extern crate rgsl;
use rgsl::legendre::associated_polynomials::legendre_Plm;

fn main() {
    println!("{}", legendre_Plm(1, 0, 1f64));
}
