extern crate cursive;
use cursive::{
    Cursive,
    views::Dialog,
};

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::text("Hello!")
            .title("Hi")
            .button("Next", show_next)
    );
    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Hmm?")
            .title("Q1")
            .button("Y", |s| show_answer(s, "Good!"))
            .button("N", |s| show_answer(s, "Bad!"))
            .button("R", |s| s.add_layer(Dialog::info("Try again!")))
    );
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Results")
            .button("Finish", |s| s.quit())
    );
} 
