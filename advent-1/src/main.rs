use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();
    println!("Enter input file: ");
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a vaild string\n");

    // Breaks if bad input
    // Bad bad bad
    let input = fs::read_to_string(s).expect("File not found");
    println!("Input: {}", &input);
}

