use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut digits: Vec<char>;
    let mut summands_count: i32;
    let mut len: usize;
    let mut summands: String;

    line = iterator.next().unwrap().unwrap();
    let n: i32 = line.parse().unwrap();

    for _i in 0..n {
        summands_count = 0;
        summands = "".to_owned();
        line = iterator.next().unwrap().unwrap();
        len = line.len();
        digits = line.chars().collect();
        for (i, c) in digits.iter().enumerate() {
            if *c != '0' {
                summands
                    .push_str(
                        &format!("{}{} ",
                            c,
                            "0".repeat(len-i-1)
                        )
                    );
                summands_count += 1;
            }
        }
        println!("{}", summands_count);
        println!("{}", summands);
    }
}