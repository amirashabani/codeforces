use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let _n: i32;
    let mut iterator = stdin.lock().lines();
    let mut line: String;

    line = iterator.next().unwrap().unwrap();
    _n = line.parse().unwrap();
    line = iterator.next().unwrap().unwrap();
    let unique = line
        .to_lowercase()
        .chars()
        .collect::<Vec<char>>()
        .iter().cloned().collect::<HashSet<_>>();

    if unique.len() as i32 == 26 {
        println!("YES");
    } else {
        println!("NO");
    }
}