use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().unwrap();
    let nm: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    for i in 1..=nm[0] {
        if i % 2 == 1 {
            for _j in 1..=nm[1] {
                print!("#");
            }
        } else {
            if (i/2) % 2 == 1 {
                for _k in 1..=(nm[1]-1) {
                    print!(".");
                }
                print!("#");
            } else {
                print!("#");
                for _k in 1..=(nm[1]-1) {
                    print!(".");
                }
            }
        }
        print!("\n");
    }
}