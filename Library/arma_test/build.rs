extern crate cpp_build;

fn main() {
    let lib_path = "/usr/lib";
    let _include_path = "/usr/include";
    // =========================================================================
    // CPP Crate
    // =========================================================================

    // cpp_build::Config::new()
    //     .opt_level(2)
    //     .include(include_path).build("src/main.rs");

    // =========================================================================
    // CC Crate
    // =========================================================================
    cc::Build::new()
        .cpp(true)
        .flag("-O3")
        .flag("-larmadillo")
        .file("src/arma.cpp")
        .compile("arma");

    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=armadillo");
}
