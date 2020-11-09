use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().unwrap();
    let mut chars: Vec<char> = line
        .chars()
        .filter(|&s| (s != ',' && s != ' '))
        .collect();

    chars.remove(0);
    chars.pop();

    let distinct: HashSet<_> = chars
        .iter().cloned().collect();

    println!("{}", distinct.len() as i32);
}