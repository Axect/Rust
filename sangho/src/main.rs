fn main() {
    let a: Vec<usize> = vec![5,10,20,4,15,990,999,87];
    println!("{}", count_zero(&a));
    println!("{}", a.into_iter().fold(1usize, |x, y| x * y));
}

fn count_zero(list: &[usize]) -> usize {
    let mut count_2 = 0usize;
    let mut count_5 = 0usize;

    for &elem in list.into_iter() {
        let mut temp = elem;
        while temp % 2 == 0 {
            count_2 += 1;
            temp /= 2;
        }

        while temp % 5 == 0 {
            count_5 += 1;
            temp /= 5;
        }
    }
    std::cmp::min(count_2, count_5)
}
