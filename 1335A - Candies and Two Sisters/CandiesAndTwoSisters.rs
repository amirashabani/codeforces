use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let (t, mut n): (i32, i32);
    let mut iterator = stdin.lock().lines();
    let mut line: String;

    line = iterator.next().unwrap().unwrap();
    t = line.parse().unwrap();

    for _i in 0..t {
        line = iterator.next().unwrap().unwrap();
        n = line.parse().unwrap();
        println!("{}", (n-1)/2);
    }
}