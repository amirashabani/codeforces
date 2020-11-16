use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i32;
    let numbers: Vec<i32>;
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut amazing: i32 = 0;
    let mut i: usize;
    let (mut min, mut max): (i32, i32);

    line = iterator.next().unwrap().unwrap();
    n = line.parse().unwrap();

    line = iterator.next().unwrap().unwrap();
    numbers = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    
    min = numbers[0];
    max = numbers[0];
    i = 1;
    while (i as i32) < n {
        if numbers[i] > max {
            amazing += 1;
            max = numbers[i];
        }
        if numbers[i] < min {
            amazing += 1;
            min = numbers[i];
        }

        i += 1;
    }

    println!("{}", amazing);
}