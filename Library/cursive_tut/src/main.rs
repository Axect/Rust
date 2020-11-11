extern crate cursive;
use cursive::{
    views::TextView,
    Cursive,
};

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(TextView::new("Hello, Cursive! Press <q> to quit."));
    siv.run();
}
