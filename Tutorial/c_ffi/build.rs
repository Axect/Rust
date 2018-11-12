extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/doubler.c")
        .compile("libdoublerc.a");
}
