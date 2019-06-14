fn main() {
    println!("{:?}", perm(5,3));
}

fn perm(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut p = (1 .. k+1).collect::<Vec<usize>>();
    let mut result: Vec<Vec<usize>> = Vec::new();

    loop {
        result.push(p.clone());
        let mut i = k;

        while i > 0 && p[i-1] == n+i-k {
            i -= 1;
        }
        if i > 0 {
            p[i-1] += 1;
            for j in i .. k {
                p[j] = p[j-1] + 1;
            }
        }
        if i == 0 {
            break;
        }
    }
    result
}
