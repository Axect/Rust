fn main() {
    let health = 32;
    let mut game = "Space Invaders";

    println!("address of health-value: {:p}", &health);
    println!("address of game-value: {:p}", &game);

    let game2 = &game;
    println!("{:p}", game2);
    println!("{}", *game2);

    let mut score = 0;
    let score2 = &mut score; // borrow score as mutable (only once!)
    *score2 = 5;
    println!("{}", *score2);
}