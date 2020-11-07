use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().unwrap();
    let colors: Vec<&str> = line
        .split_whitespace()
        .collect();
    
    let unique: HashSet<_> = colors.iter().cloned().collect();
    
    let unique_count: i32 = unique.len() as i32;
    println!("{}", (unique_count-4).abs());
}