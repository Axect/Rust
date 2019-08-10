fn main() {
    let num = 5;
    let num2 = 4;

    let r1 = &num as *const i32;
    let mut r2 = &num as *const i32;

    unsafe {
        r2 = r1;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
