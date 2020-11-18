use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let (_n, max): (i32, i32);
    let numbers: Vec<i32>;
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut min: i32;

    line = iterator.next().unwrap().unwrap();
    _n = line.parse().unwrap();

    line = iterator.next().unwrap().unwrap();
    numbers = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    match numbers.iter().max() {
        Some(number) => max = *number,
        None => max = 0,
    }

    min = 0;
    for number in numbers.iter() {
        min += max - number;
    }

    println!("{}", min);
}