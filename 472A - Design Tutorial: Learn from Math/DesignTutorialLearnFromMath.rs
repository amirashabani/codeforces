use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().unwrap();
    let n: i32 = line.parse().unwrap();

    let mut i: i32 = 4;
    let mut found: bool = false;
    while i <= (n-4) && !found {
        if !is_prime(i) && !is_prime(n-i) {
            found = true;
        }
        i += 1;
    }

    i -= 1;
    println!("{} {}", i, n-i);
}

fn is_prime(number: i32) -> bool {
    if number <= 3 {
        return number > 1;
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let mut i: i32 = 5;
    while i * i <= number {
        if number % i == 0 || number % (i+2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}