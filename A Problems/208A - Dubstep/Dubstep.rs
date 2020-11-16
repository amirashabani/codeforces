use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().unwrap();

    let words: Vec<&str> = line
        .split("WUB")
        .filter(|s| !s.is_empty())
        .collect();
    
    println!("{}", words.join(" "));
}