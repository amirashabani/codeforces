use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i32;
    let line: String;
    let mut iterator = stdin.lock().lines();
    let mut cubes: i32;
    let mut height: i32;

    line = iterator.next().unwrap().unwrap();
    n = line.parse().unwrap();

    height = 1;
    cubes = 0;
    while cubes <= n {
        cubes += (height*(height+1))/2;
        height += 1;
    }

    println!("{}", height-2);
}