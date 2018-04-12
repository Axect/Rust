static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";

fn main() {
    const PI: f32 = 3.14;
    println!("The Game you are playing is called {}.", GAME_NAME);
    println!("You start with {} health points.", MAX_HEALTH);
    println!("You have {points} % health", points=70);
    println!("pi is {:e} in floating point notation", PI);
}
