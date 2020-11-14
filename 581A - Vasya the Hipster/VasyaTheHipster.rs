use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line: String;
    let numbers: Vec<i32>;
    let (min, max): (i32, i32);

    line = iterator.next().unwrap().unwrap();
    numbers = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    match numbers.iter().min() {
        Some(number) => min = *number,
        None => min = 0,
    }
    match numbers.iter().max() {
        Some(number) => max = *number,
        None => max = 0,
    }

    println!("{} {}", min, (max-min)/2);
}