use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line: String;
    let nk: Vec<i32>;
    let left: i32;
    let mut iterator = stdin.lock().lines();
    let mut spent: i32 = 0;
    let mut solved: i32 = 0;

    line = iterator.next().unwrap().unwrap();
    nk = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    left = 240 - nk[1];
    while spent+((solved+1)*5) <= left && solved+1 <= nk[0] {
        solved += 1;
        spent += solved*5;
    }

    println!("{}", solved);
}