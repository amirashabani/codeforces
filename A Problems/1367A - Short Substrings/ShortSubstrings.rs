use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let t: i32;
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut original: Vec<String> = Vec::new();

    line = iterator.next().unwrap().unwrap();
    t = line.parse().unwrap();

    for _ in 0..t {
        original.clear();
        line = iterator.next().unwrap().unwrap();
        for (i, c) in line.chars().enumerate() {
            if i <= 1 || i % 2 == 1 {
                original.push(c.to_string());
            }
        }
        println!("{}", original.join(""));
    }
}