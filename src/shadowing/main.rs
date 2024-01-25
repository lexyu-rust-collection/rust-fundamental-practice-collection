#![allow(unused)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::mem;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "100";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    // let size_in_bytes = mem::size_of::<u32>();
    // println!("Size of u32 in bytes: {}", size_in_bytes);
}
