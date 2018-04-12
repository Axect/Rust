fn main() {
    let points = -10i32;
    let mut saved_points: u32 = 0;
    println!("save point: {}", saved_points);

    saved_points = points as u32; // Explicit Transform
    println!("save point: {}", saved_points);
}