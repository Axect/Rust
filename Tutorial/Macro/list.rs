pub fn main() {
    let a = c![1,2,3];
    println!("{:?}", a);
}

#[macro_export]
macro_rules! c {
    ( $($x:expr),* ) => {
        { 
            let mut v: Vec<f64> = Vec::new();
            $(
                v.push($x as f64);
            )*
            v
        }
    };
}
