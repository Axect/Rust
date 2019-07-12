#![feature(proc_macro_hygiene)]
use inline_python::python;

fn main() {
    let who = "world";
    let n = 5;
    let x = vec![1f64,2f64,3f64];
    python! {
        import pylab as plt
        plt.plot('x, 'x)
        for i in range('n):
            print(i, "Hello", 'who)
        print("Goodbye")
        plt.show()
    }
}
