use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i32;
    let mut iterator = stdin.lock().lines();
    let mut home: Vec<i32> = Vec::new();
    let mut guest: Vec<i32> = Vec::new();
    let mut colors: Vec<i32>;
    let mut line: String;
    let mut count: i32 = 0;

    line = iterator.next().unwrap().unwrap();
    n = line.parse().unwrap();

    for _i in 0..n {
        line = iterator.next().unwrap().unwrap();
        colors = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        home.push(colors[0]);
        guest.push(colors[1]);
    }

    for color in home.iter() {
        count += guest
            .iter()
            .filter(|&n| *n == *color)
            .count() as i32;
    }

    println!("{}", count);
}