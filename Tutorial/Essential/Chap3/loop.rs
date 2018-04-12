fn main() {
    let max_power = 10;
    let mut power = 1;
    while power < max_power {
        print!("{} ", power);
        power += 1;
    }

    loop {
        power += 1;
        if power == 42 {
            continue;
        }
        print!("{} ", power);
        if power == 50 {
            println!("OK, that's enough for today");
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer dungeon - ");
        'inner:loop {
            println!("Entered the inner dungeon - ");
            break 'outer;
        }
        println!("This treasure can sadly never be reached - ");
    }
    println!("Exited the outer dungeon!");

    for n in 1..11 {
        println!("The square of {} is {}", n, n*n);
    }

    let mut x = 10;
    for _ in 1 .. x {x -= 1; print!("."); }
}