pub fn ans() -> u64 {
    (1000 .. 1_000_000)
        .filter(|&x| is_palindrome(x))
        .filter(|&x| is_product(Digit::Three, x))
        .max().unwrap()
}

#[derive(Debug)]
pub enum Digit {
    Two,
    Three,
}

pub fn is_palindrome(n: u64) -> bool {
    let s: String = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn is_product(d: Digit, n: u64) -> bool {
    match d {
        Digit::Two => {
            (10 .. 100)
                .filter(|x| n % x == 0)
                .any(|x| n / x >= 10 && n / x < 100)
        },
        Digit::Three => {
            (100 .. 1000)
                .filter(|x| n % x == 0)
                .any(|x| n / x >= 100 && n / x < 1000)
        },
    }
}