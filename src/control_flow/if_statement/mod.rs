#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::mem;

pub fn output() {
    let age = rand::thread_rng().gen_range(1..101);
    println!("age : {}", age);

    if (age >= 1) && (age <= 18) {
        println!("in the range");
    } else {
        println!("not in the range")
    }
}
