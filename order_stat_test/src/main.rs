extern crate peroxide;
extern crate order_stat;

use peroxide::*;
use order_stat::*;
use QType::*;

fn main() {
    let u = Uniform(1, 10);
    let mut u_set = u.sample(20);

    u_set.print();
    quantiles(&mut u_set, Type1).print();
    quantiles(&mut u_set, Type2).print();


    //let mut df = DataFrame::with_header(vec!["u"]);
    //df["u"] = u_set;
    //df.write_nc("u.nc").expect("Can't write nc");
}

#[derive(Debug, Copy, Clone)]
enum QType {
    Type1,
    Type2,
    Type3
}

fn quartile_mut(v: &mut Vec<f64>, q: f64, t: QType) -> f64 {
    let l = v.len();
    let p = 1f64 / (l as f64);
    let k = (q / p) as usize;
    match t {
        Type1 => {
            let k = if q == 0f64 {
                0
            } else if q - (k as f64) * p > 0f64 { 
                k 
            } else { 
                k - 1 
            };
            *kth_by(v, k, |x,y| x.partial_cmp(y).unwrap())
        }
        Type2 => {
            if q - (k as f64) * p > 0f64 { 
                *kth_by(v, k, |x,y| x.partial_cmp(y).unwrap())
            } else if q == 0f64 {
                let k = 0;
                *kth_by(v, k, |x,y| x.partial_cmp(y).unwrap())
            } else if q == 1f64 {
                let k = l - 1;
                *kth_by(v, k, |x,y| x.partial_cmp(y).unwrap())
            } else { 
                let prev = *kth_by(v, k-1, |x,y| x.partial_cmp(y).unwrap());
                let next = *kth_by(v, k, |x,y| x.partial_cmp(y).unwrap());
                (prev + next) / 2f64
            }
        }
        _ => unimplemented!()
    }
}

fn quantiles(v: &mut Vec<f64>, t: QType) -> Vec<f64> {
    let q_vec = c!(0.0, 0.25, 0.5, 0.75, 1.0);
    let mut result = vec![0f64; 5];
    for (i, q) in q_vec.into_iter().enumerate() {
        result[i] = quartile_mut(v, q, t);
    }
    result
}
