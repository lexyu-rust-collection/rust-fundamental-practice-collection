#![allow(unused)]

pub fn output() {
    let int1_u8: u8 = 5;
    let int2_u8: u8 = 5;
    // let int_u32: u32 = int1_u8 + int2_u8; // error

    let int_u32: u32 = (int1_u8 as u32) + (int2_u8 as u32);
    println!("{int_u32}");
}
