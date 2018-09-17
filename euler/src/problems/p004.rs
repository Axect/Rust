pub fn ans() -> u64 {
    Palindrome { num: 10000 }
        .take_while(|&x| x < 1_000_000)
        .filter(|&x| is_product(Digit::Three, x))
        .max().unwrap()
}

pub enum Digit {
    Two,
    Three,
}

pub struct Palindrome {
    num: u64,
}

impl Iterator for Palindrome {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.num += 1;
        while !is_palindrome(self.num) {
            self.num += 1;
        }
        Some(self.num)
    }
}

pub fn is_palindrome(n: u64) -> bool {
    let s: String = n.to_string();
    for (i1, i2) in s.chars().zip(s.chars().rev()) {
        if i1 != i2 {
            return false;
        }
    }
    return true;
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