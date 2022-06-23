use peroxide::fuga::*;
use special_fun::unsafe_cephes_double::hyp2f1;
use rgsl::hypergeometric::hyperg_2F1;

fn main() {
    let k = 2.0;
    let p = 1f64;
    let q = 1000f64;

    let gk = f_gk(k, (p, q));
    let hyp = f_hyp2f1(k, (p, q));
    let hypg = f_hyperg(k, (p, q));

    gk.print();
    hyp.print();
    hypg.print();
    
    //let mut r = 0f64;
    //for i in 0 .. 10000 {
    //    let hypg = f_hyperg(k, (p, q));
    //    r += hypg;
    //}
    //r.print();
}

fn f_gk(k: f64, (p, q): (f64, f64)) -> f64 {
    let f = |x: f64| 1f64 / (1f64 + k * (1f64 / x).powf(4f64/3f64)).sqrt();

    integrate(f, (p, q), G7K15(1e-10))
}

fn f_hyp2f1(k: f64, (p, q): (f64, f64)) -> f64 {
    let a = 1f64;
    let b = 0.5;
    let c = 0.25;
    let z_p = -k / p.powf(4f64/3f64);
    let z_q = -k / q.powf(4f64/3f64);
    let z_p2 = z_p / (z_p - 1f64);
    let z_q2 = z_q / (z_q - 1f64);

    let f_q2 = if z_q2 < 1f64 {
        unsafe {
            q / (1f64 - z_q).sqrt() * hyp2f1(a, b, c, z_q2)
        }
    } else if z_q2 == 1f64 {
        0f64   
    } else if z_q2 == 0f64 {
        1f64
    } else {
        panic!("z_q2 is not in [0, 1]")
    };

    let f_p2 = if z_p2 < 1f64 {
        unsafe {
            p / (1f64 - z_p).sqrt() * hyp2f1(a, b, c, z_p2)
        }
    } else if z_p2 == 1f64 {
        0f64   
    } else if z_p2 == 0f64 {
        1f64
    } else {
        panic!("z_p2 is not in [0, 1]")
    };

    f_q2 - f_p2
}

fn f_hyperg(k: f64, (p, q): (f64, f64)) -> f64 {
    let a = 1f64;
    let b = 0.5;
    let c = 0.25;
    let z_p = -k / p.powf(4f64/3f64);
    let z_q = -k / q.powf(4f64/3f64);
    let z_p2 = z_p / (z_p - 1f64);
    let z_q2 = z_q / (z_q - 1f64);
    
    let f_q2 = if z_q2 < 1f64 {
        q / (1f64 - z_q).sqrt() * hyperg_2F1(a, b, c, z_q2)
    } else if z_q2 == 1f64 {
        0f64   
    } else if z_q2 == 0f64 {
        1f64
    } else {
        panic!("z_q2 is not in [0, 1]")
    };

    let f_p2 = if z_p2 < 1f64 {
        p / (1f64 - z_p).sqrt() * hyperg_2F1(a, b, c, z_p2)
    } else if z_p2 == 1f64 {
        0f64   
    } else if z_p2 == 0f64 {
        1f64
    } else {
        panic!("z_p2 is not in [0, 1]")
    };

    f_q2 - f_p2
}
