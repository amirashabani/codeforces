use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let (t, mut n): (i32, i32);
    let mut iterator = stdin.lock().lines();
    let mut line: String;
    let mut even: Vec<String> = Vec::new();
    let mut odd: Vec<String> = Vec::new();

    line = iterator.next().unwrap().unwrap();
    t = line.parse().unwrap();

    for _ in 0..t {
        even.clear();
        odd.clear();
        line = iterator.next().unwrap().unwrap();
        n = line.parse().unwrap();

        if (n/2)%2 == 0 {
            println!("YES");
            for i in 1..=(n/2) {
                even.push((i*2).to_string());
                odd.push(((i*2)-1).to_string());
            }
            odd[((n/2)-1) as usize] = ((n-1)+(n/2)).to_string();
            println!("{} {}",
                even.join(" "),
                odd.join(" ")
            )
        } else {
            println!("NO");
        }
    }
}