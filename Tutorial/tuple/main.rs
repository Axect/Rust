#[derive(PartialEq)]
struct Fahrenheit(i64);

#[derive(PartialEq)]
struct Celsius(i64);

fn main() {
    let temperature1 = Fahrenheit(10);
    let temperature2 = Celsius(10);

    println!("Is temperature1 the same as temperature2? Answer: {}", temperature1 == temperature2);
    println!("Temperature1 is {} fahrenheit", temperature1.0);
    println!("Temperature2 is {} celsius", temperature2.0);
}
