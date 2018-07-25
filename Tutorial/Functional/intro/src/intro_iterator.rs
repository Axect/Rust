pub fn iter1() -> u32 {
    (0..10).map(|x| x*x)
           .inspect(|x|{ println!("value {}", *x) })
           .filter(|x| *x < 10)
           .filter_map(|x| Some(x))
           .fold(0, |x,y| x+y)
}