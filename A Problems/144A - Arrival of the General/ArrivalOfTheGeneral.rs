use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut time: i32 = 0;

    let first = iterator.next().unwrap().unwrap();
    let second = iterator.next().unwrap().unwrap();

    let n: i32 = first.parse().unwrap();
    let heights: Vec<i32> = second
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let min = heights.iter().min().unwrap();
    let max = heights.iter().max().unwrap();

    let minp = heights
        .iter()
        .rposition(|r| r == min)
        .unwrap();
    let maxp = heights
        .iter()
        .position(|r| r == max)
        .unwrap();
    
    time += maxp as i32;
    time += n - (minp as i32) - 1;

    if maxp > minp {
        time -= 1;
    }
    
    println!("{}", time);
}