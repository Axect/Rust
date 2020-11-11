extern crate flot;
extern crate peroxide;
use peroxide::*;

fn main() {
    let x = seq(0, 10, 0.1);
    let y = x.fmap(|x| x.powi(2));

    let data = x.into_iter().zip(y).collect::<Vec<(f64, f64)>>();

    let page = flot::Page::new("");

    let p = page.plot("line");
    p.lines("lines", data.clone()).line_width(1);

    page.render("simple.html").expect("io error");
}
