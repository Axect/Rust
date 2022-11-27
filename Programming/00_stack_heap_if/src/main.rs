fn main() {
    println!("Hello, world!");
}

#[test]
fn stack() {
    let x = [1,2,3,4];

    if x.len() % 2 == 0 {
        let y = x;
    } else {
        println!("{:?}", x);
    }

    dbg!(x);
}

#[test]
#[should_panic]
fn heap() {
    let x = vec![1,2,3,4];

    if x.len() % 2 == 0 {
        let y = x;
    } else {
        println!("{:?}", x);
    }

    dbg!(x);
}
