fn main() {
    println!("{}", diff(f, 1f64, 1e-6));
    let df = Derivative {
        f,
        h: 1e-6,
    };
    println!("{}", df.calc(1f64));

    let dg = derivative(f, 1e-6);
    println!("{}", dg(1f64));
}

fn diff<F: Fn(f64) -> f64>(f: F, x: f64, h: f64) -> f64 {
    (f(x+h)-f(x)) / h
}

fn f(x: f64) -> f64 {
    x.powi(2)
}

struct Derivative<F: Fn(f64) -> f64> {
    pub f: F,
    pub h: f64,
}

impl<F: Fn(f64) -> f64> Derivative<F> {
    fn f(&self, x: f64) -> f64 {
        (self.f)(x)
    }

    fn calc(&self, x: f64) -> f64 {
        (self.f(x+self.h) - self.f(x)) / self.h
    }
}

fn derivative<F>(f: F, h: f64) -> impl Fn(f64) -> f64
where
    F: Fn(f64) -> f64,
{
    move |x: f64| (f(x+h) - f(x)) / h
}
