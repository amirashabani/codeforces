use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut count: i32 = 0;

    let first = iterator.next().unwrap().unwrap();
    let n: i32 = first.parse().unwrap();
    let mut line: String;

    let polyhedrons: HashMap<&str, i32> = [
            ("Tetrahedron", 4),
            ("Cube", 6),
            ("Octahedron", 8),
            ("Dodecahedron", 12),
            ("Icosahedron", 20),
        ].iter().cloned().collect();

    for _i in 0..n {
        line = iterator.next().unwrap().unwrap();
        count += polyhedrons.get(&line.as_str()).unwrap();
    }

    println!("{}", count);
}