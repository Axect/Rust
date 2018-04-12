fn main() {
    let points = -10i32;
    let mut saved_points: u32 = 0;
    println!("save point: {}", saved_points);

    saved_points = points as u32; // Explicit Transform
    println!("save point: {}", saved_points);

    let f2 = 3.14;
    saved_points = f2 as u32;
    println!("save point: {}", saved_points);
}