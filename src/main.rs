#![allow(unused)]

mod control_flow;
mod data_types;
mod io_demo;
mod math;
mod shadowing;

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::mem;

fn main() {
    // control_flow::if_statement::output();
    // control_flow::terrnary_operator::output();
    control_flow::match_demo::output();

    // data_types::numbers_demo::output();
    // data_types::string_demo::output();

    // math::mod_math::output();

    // io_demo::stdin::output();
    // shadowing::shadow::output();
}
