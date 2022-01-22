fn main() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 132, 203, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
