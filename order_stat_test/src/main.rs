extern crate peroxide;
extern crate statrs;

use peroxide::*;
use statrs::statistics::OrderStatistics;

fn main() {
    let u = Uniform(1.0, 10.0);
    let mut u_set = u.sample(100);

    u_set.lower_quartile().print();
    u_set.median().print();
    u_set.upper_quartile().print();

    let mut df = DataFrame::with_header(vec!["u"]);
    df["u"] = u_set;
    df.write_nc("u.nc").expect("Can't write nc");
}
