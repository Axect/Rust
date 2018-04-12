fn main() {
    let energy = 5u16; // _energy means unused variable
    let _copy_energy = energy;
    println!("Your energy is {}", energy);

    let _level_title = "Level 1";
    let _dead = false;
    let _magic_number = 3.14f32;
    let _empty = ();

    let fuel: i64;
    fuel = 60;
    println!("fuel: {}", fuel);

    let n: i32 = -2;
    println!("n: {}", n);

    let outer = 42;
    {
        let inner = 3.14;
        println!("block variable: {}", inner);
        let outer = 99;
        println!("block variable outer: {}", outer);
    }
    println!("outer variable: {}", outer);

    let player1 = "jane";
    let player2 = "tom";
    let player3 = format!("{}{}", player1, player2);

    print!("{}", player3);
}