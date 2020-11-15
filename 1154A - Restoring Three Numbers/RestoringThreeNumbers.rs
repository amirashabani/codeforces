use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line: String;
    let mut iterator = stdin.lock().lines();
    let mut numbers: Vec<i32>;
    let (a, b, c): (i32, i32, i32);

    line = iterator.next().unwrap().unwrap();
    numbers = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    numbers.sort();
    
    a = (numbers[0]+numbers[1]-numbers[2])/2;
    b = (numbers[1]+numbers[2]-numbers[0])/2;
    c = (numbers[2]+numbers[0]-numbers[1])/2;

    println!("{} {} {}", a, b, c);
}