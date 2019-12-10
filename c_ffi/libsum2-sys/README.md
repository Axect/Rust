# Link Fortran code

## 0. Source tree

* `src`
    * `main.rs` : Main function
    * `c_code`
        * `sum2.f90` : Fortran code to link
        * `sum2.c` : C wrapper

## 1. Check `sum2.f90`

```fortran
subroutine sum2(x, y)
implicit none
Integer,intent(in) :: x
Integer,intent(out) :: y
y = x + 2
end subroutine sum2
```

## 2. Write C wrapper

```c
#include <stdio.h>

extern void sum2_(int *x, int *y);

int sum2(int x) {
    int y = 0;
    sum2_(&x, &y);
    return y;
}
```

## 3. Compile fortran to object

```sh
gfortran -c sum2.f90
```

## 4. Write `Cargo.toml`

```toml
[package]
name = "libsum2-sys"
version = "0.1.0"
build = "build.rs"
authors = ["Axect <axect@outlook.kr>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
libc = "0.2"

[build-dependencies]
cc = { version = "1", features = ["parallel"] }
```

## 5. Write `build.rs`

```rust
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_code/sum2.c")
        .object("src/c_code/sum2.o")
        .compile("sum2")
}
```

## 6. Write `main.rs`

```rust
extern crate libc;
use libc::c_int;

#[link(name = "sum2")]
extern {
    fn sum2(x: c_int) -> c_int;
}

fn main() {
    unsafe {
        println!("{}", sum2(1));
    }
}
```