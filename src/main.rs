use std::io;
use std::fs;

fn main() {
    //Take inputs and store in Vector separated by spaces
    let mut input = String::new();
    println!("Input multiple strings sparated by spaces");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let n = input[0];
    println!("First item: {}", n);
    println!("Number of inputs: {}", input.len());
}
