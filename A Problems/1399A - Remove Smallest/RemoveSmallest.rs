use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let (t, mut n, mut i): (i32, i32, usize);
    let mut iterator = stdin.lock().lines();
    let mut numbers: Vec<i32>;
    let mut possible: bool;
    let mut line: String;

    line = iterator.next().unwrap().unwrap();
    t = line.parse().unwrap();

    for _ in 0..t {
        line = iterator.next().unwrap().unwrap();
        n = line.parse().unwrap();

        line = iterator.next().unwrap().unwrap();
        numbers = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        
        numbers.sort();

        i = 0;
        possible = true;
        while (i as i32) < (n-1) && possible {
            if numbers[i+1] - numbers[i] > 1 {
                possible = false;
            }
            i += 1;
        }

        if possible {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}