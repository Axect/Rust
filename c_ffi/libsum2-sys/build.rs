extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_code/sum2.c")
        .object("src/c_code/sum2.o")
        .compile("sum2")
}