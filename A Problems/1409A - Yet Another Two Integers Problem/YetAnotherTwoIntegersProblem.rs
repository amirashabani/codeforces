use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let t: i32;
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut ab: Vec<f64>;
    let mut min: i32;

    line = iterator.next().unwrap().unwrap();
    t = line.parse().unwrap();

    for _ in 0..t {
        line = iterator.next().unwrap().unwrap();
        ab = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        
        min = ((ab[0] - ab[1]).abs() / 10.0).ceil() as i32;

        println!("{}", min);
    }
}