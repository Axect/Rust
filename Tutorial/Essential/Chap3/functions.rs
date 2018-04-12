fn main() {
    let hero1 = "Pac Man";
    let hero2 = "Riddick";
    greet(hero2);
    greet_both(hero1, hero2);
}

fn greet(name: &str) {
    println!("Hi mighty {}, what brings you here?", name);
}

fn greet_both(name1: &str, name2: &str) {
    greet(name1);
    greet(name2);
}