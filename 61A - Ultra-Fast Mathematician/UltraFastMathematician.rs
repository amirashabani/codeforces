use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let first = iterator.next().unwrap().unwrap();
    let second = iterator.next().unwrap().unwrap();

    let first_vec: Vec<char> = first.chars().collect();
    let second_vec: Vec<char> = second.chars().collect();

    let mut result = Vec::new();

    for i in 0..first_vec.len() {
        if first_vec[i] != second_vec[i] {
            result.push("1");
        } else {
            result.push("0");
        }
    }

    println!("{}", result.join(""));
}