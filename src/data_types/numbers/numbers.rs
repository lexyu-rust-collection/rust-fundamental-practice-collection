#![allow(unused)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::mem;

fn main() {
    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize

    println!("Max u8 : {}", u8::MAX);
    println!("Max u16 : {}", u16::MAX);
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("MIN u8 : {}", u8::MIN);
    println!("MIN u16 : {}", u16::MIN);
    println!("MIN u32 : {}", u32::MIN);
    println!("MIN u64 : {}", u64::MIN);
    println!("MIN u128 : {}", u128::MIN);
    println!("MIN usize : {}", usize::MIN);
    println!("BITS u8 : {}", u8::BITS);
    println!("BITS u16 : {}", u16::BITS);
    println!("BITS u32 : {}", u32::BITS);
    println!("BITS u64 : {}", u64::BITS);
    println!("BITS u128 : {}", u128::BITS);
    println!("BITS usize : {}", usize::BITS);

    println!("Max i8 : {}", i8::MAX);
    println!("Max i16 : {}", i16::MAX);
    println!("Max i32 : {}", i32::MAX);
    println!("Max i64 : {}", i64::MAX);
    println!("Max i128 : {}", i128::MAX);
    println!("Max isize : {}", isize::MAX);
    println!("MIN i8 : {}", i8::MIN);
    println!("MIN i16 : {}", i16::MIN);
    println!("MIN i32 : {}", i32::MIN);
    println!("MIN i64 : {}", i64::MIN);
    println!("MIN i128 : {}", i128::MIN);
    println!("MIN isize : {}", isize::MIN);
    println!("BITS i8 : {}", i8::BITS);
    println!("BITS i16 : {}", i16::BITS);
    println!("BITS i32 : {}", i32::BITS);
    println!("BITS i64 : {}", i64::BITS);
    println!("BITS i128 : {}", i128::BITS);
    println!("BITS isize : {}", isize::BITS);

    println!("MAX f32 : {}", f32::MAX);
    println!("MAX f64 : {}", f64::MAX);
    println!("MIN f32 : {}", f32::MIN);
    println!("MIN f64 : {}", f64::MIN);
}
