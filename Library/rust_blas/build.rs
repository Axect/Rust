fn main() {
    println!("cargo:rustc-link-search=/opt/OpenBLAS/lib");
    println!("cargo:rustc-link-lib=dylib=gfortran");
    println!("cargo:rustc-link-lib=dylib=openblas");
}
