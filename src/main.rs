use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read error");
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let sum: i64 = numbers.into_iter().sum();
    println!("{}", sum);
}
