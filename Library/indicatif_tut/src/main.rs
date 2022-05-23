use indicatif::ProgressBar;
use peroxide::fuga::*;

fn main() {
    let a = rand(1000, 1000);
    let b = rand(1000, 1);
    let mut c = vec![0f64; 1000];

    let pb = ProgressBar::new(1000);

    for i in 0 .. 1000 {
        pb.inc(1);
        c[i] = (&a * &b)[(i,0)];
    }

    pb.finish_with_message("done");

    let mut df = DataFrame::new(vec![]);
    df.push("c", Series::new(c));

    df.print();
}
