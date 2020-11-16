use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut damaged = 0;

    let k: i64 = iterator
        .next().unwrap().unwrap().parse().unwrap_or(0);
    let l: i64 = iterator
        .next().unwrap().unwrap().parse().unwrap_or(0);
    let m: i64 = iterator
        .next().unwrap().unwrap().parse().unwrap_or(0);
    let n: i64 = iterator
        .next().unwrap().unwrap().parse().unwrap_or(0);
    let d: i64 = iterator
        .next().unwrap().unwrap().parse().unwrap_or(0);

    for i in 1..=d {
        if i % k == 0 ||
            i % l == 0 ||
            i % m == 0 ||
            i % n == 0 {
                damaged += 1;
        }
    }

    println!("{}", damaged);
}