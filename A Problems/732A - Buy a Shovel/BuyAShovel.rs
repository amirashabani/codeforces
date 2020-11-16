use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let kr: Vec<i32>;
    let line: String;
    let mut iterator = stdin.lock().lines();
    let mut min: i32;

    line = iterator.next().unwrap().unwrap();
    kr = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    min = 1;
    while (kr[0]*min)%10 != 0 && (kr[0]*min)%10 != kr[1] {
        min += 1;
    }

    println!("{}", min);
}