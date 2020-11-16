use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first = iterator.next().unwrap().unwrap();
    let second = iterator.next().unwrap().unwrap();

    let n: i32 = first.parse().unwrap();

    let numbers: Vec<i32> = second
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let sum: i32 = numbers.iter().sum();

    println!("{}", (sum as f64) / (n as f64));
}