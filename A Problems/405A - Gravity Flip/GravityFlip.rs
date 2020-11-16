use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let _first = iterator.next().unwrap().unwrap();
    let second = iterator.next().unwrap().unwrap();

    let mut numbers: Vec<i32> = second
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    numbers.sort();

    let strings: Vec<String> = numbers
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("{}", strings.join(" "));
}