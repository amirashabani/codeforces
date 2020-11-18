use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let numbers: Vec<i32>;
    let _n: i32;
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut untreated: i32;
    let mut officers: i32;

    line = iterator.next().unwrap().unwrap();
    _n = line.parse().unwrap();

    line = iterator.next().unwrap().unwrap();
    numbers = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    untreated = 0;
    officers = 0;
    for number in numbers.iter() {
        if *number == -1 {
            if officers == 0 {
                untreated += 1;
            } else {
                officers -= 1;
            }
        } else {
            officers += number;
        }
    }

    println!("{}", untreated);
}