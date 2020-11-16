use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first = iterator.next().unwrap().unwrap();
    let n: i32 = first.parse().unwrap();

    let mut line: String;
    let mut ab: Vec<i64>;
    let mut reminder: i64;
    let mut minimum: i64;

    for _i in 0..n {
        line = iterator.next().unwrap().unwrap();
        ab = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        reminder = ab[0] % ab[1];
        if reminder == 0 {
            minimum = 0;
        } else {
            minimum = ab[1] - reminder;
        }

        println!("{}", minimum);
    }
}