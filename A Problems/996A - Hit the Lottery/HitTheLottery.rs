use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let bills = vec![100, 20, 10, 5, 1];
    let mut n: i32;
    let line: String;
    let mut min: i32 = 0;

    line = iterator.next().unwrap().unwrap();
    n = line.parse().unwrap();

    for bill in bills.iter() {
        min += n/bill;
        n %= bill;
    }

    println!("{}", min);
}