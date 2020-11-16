use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut first: Vec<char> = Vec::new();
    let mut second: Vec<char> = Vec::new();

    line = iterator.next().unwrap().unwrap();
    first.extend(line.chars());

    line = iterator.next().unwrap().unwrap();
    first.extend(line.chars());

    line = iterator.next().unwrap().unwrap();
    second.extend(line.chars());

    first.sort();
    second.sort();

    if first == second {
        println!("YES");
    } else {
        println!("NO");
    }
}