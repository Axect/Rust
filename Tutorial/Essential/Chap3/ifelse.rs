fn main() {
    let dead = false;
    let health = 48;
    if dead {
        println!("Game over!");
    } else {
        println!("You still have a chance to win!");
    }

    if health >= 50 {
        println!("Continue to fight!");
    } else if health >= 20 {
        println!("Stop the battle and gain strength!");
    } else {
        println!("Hide and try to recover!");
    }

    // Can binding expressions (Do not use semi colon)
    let active =
    if health >= 50 {
        true
    } else {
        false
    };

    println!("Am I Active? {:?}", active);
}