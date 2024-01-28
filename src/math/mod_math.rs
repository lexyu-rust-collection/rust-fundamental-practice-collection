#![allow(unused)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::mem;

pub fn output() {
    let num_1: f32 = 1.1111111111111111111;
    println!("f32: {}", num_1 + 0.1111111111111111111); // .7f
                                                        // 1.2222223

    let num_2: f64 = 1.1111111111111111111;
    println!("f64: {}", num_2 + 0.1111111111111111111); // .16f
                                                        // 1.2222222222222223

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    num_3 += 1;

    print!("{num_3}");
}
