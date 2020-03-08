extern crate cpp_build;

fn main() {
    let lib_path = "/usr/lib";
    let include_path = "/usr/include";

    cpp_build::Config::new()
        .opt_level(2)
        .include(include_path).build("src/main.rs");

    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=armadillo");
}
