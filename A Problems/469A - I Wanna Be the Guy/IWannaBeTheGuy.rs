use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let n: usize = iterator
        .next().unwrap().unwrap().parse().unwrap_or(0);
    
    let second = iterator.next().unwrap().unwrap();
    let third = iterator.next().unwrap().unwrap();
    
    let mut p_line: Vec<&str> = second
        .split_whitespace()
        .collect();

    let mut q_line: Vec<&str> = third
        .split_whitespace()
        .collect();

    p_line.remove(0);
    q_line.remove(0);

    let p: HashSet<_> = p_line.iter().cloned().collect();
    let q: HashSet<_> = q_line.iter().cloned().collect();

    let can_pass: Vec<_> = p.union(&q).collect();

    if can_pass.len() == n {
        println!("I become the guy.");
    } else {
        println!("Oh, my keyboard!");
    }
}