use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut mins = Vec::new();

    let first = iterator.next().unwrap().unwrap();
    let second = iterator.next().unwrap().unwrap();

    let nm: Vec<usize> = first
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let mut sizes: Vec<i32> = second
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    sizes.sort();

    for i in 0..(nm[1] - nm[0] + 1) {
        mins.push(sizes[i+nm[0]-1] - sizes[i])
    }

    let min = mins.iter().min();

    match min {
        Some(value) => println!("{}", value),
        None => println!("Vector is empty."),
    }
}