#![allow(unused)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What's your name?");
    let mut name = String::new();

    let greeting = "Nice to meet you";

    io::stdin().read_line(&mut name).expect("Empty Input");

    println!("Hello, {} {}", name.trim_end(), greeting);
}
