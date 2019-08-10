fn main() {
    // Basic Raw Pointer ops
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;

    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    unsafe {
        *ptr_x += *ptr_y;
    }
    println!("x: {}, y:{}", x, y);

    // Distance
    let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
    let first = &trucks[0];
    let last = &trucks[2];
    println!("dist: {}", distance(last, first));
    println!("inv dist: {}", distance(first, last));
}

fn distance<T>(left: *const T, right: *const T) -> isize {
    (left as isize - right as isize) / std::mem::size_of::<T>() as isize
}
