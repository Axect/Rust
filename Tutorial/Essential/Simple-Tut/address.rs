fn main() {
    let health = 32;
    let mut game = "Space Invaders";

    println!("address of health-value: {:p}", &health);
    println!("address of game-value: {:p}", &game);
}