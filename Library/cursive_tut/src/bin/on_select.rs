extern crate cursive;
use cursive::traits::*;
use cursive::Cursive;
use cursive::views::{Dialog, SelectView, ListView, EditView, LinearLayout, TextView};

fn main() {
    let mut s = Cursive::default();
    let data: Vec<usize> = vec![1, 1];
    s.set_user_data(data);
    
    s.add_layer(
        Dialog::new()
            .title("Test")
            .content(
                LinearLayout::horizontal()
                    .child(
                        ListView::new()
                            .child(
                                "Select",
                                SelectView::new()
                                    .popup()
                                    .item("One", 1)
                                    .item("Two", 2)
                                    .on_submit(move |s, item| {
                                        s.with_user_data(|ud: &mut Vec<usize>| {
                                            (*ud)[0] = *item;
                                        }).unwrap()
                                    })
                            )
                            .child(
                                "Edit",
                                EditView::new()
                                    .on_submit(move |s, txt| {
                                        let ud: &mut Vec<usize> = s.user_data().unwrap();
                                        let num: usize = txt.parse().unwrap();
                                        (*ud)[1] = num;
                                    })
                            )
                    )
                    .child(
                        TextView::new("Hi")
                            .with_name("text")
                    )
            )
            .button("Ok", |s| {
                let ud: Vec<usize> = s.take_user_data().unwrap();
                s.pop_layer();
                s.add_layer(
                    Dialog::info(format!("{:?}", ud))
                )
            })
    );
    s.run();
}
