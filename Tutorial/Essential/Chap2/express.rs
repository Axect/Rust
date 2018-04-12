fn main() {
    let n1 = {
        let a = 2;
        let b = 5;
        a + b // No Semi Colon!
    };

    println!("n1 is: {}", n1);

    let n2 = {
        let a = 2;
        let b = 5;
        a + b;
    };

    println!("n2 is: {:?}", n2);
}