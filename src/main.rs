#![allow(unused)]

mod casting;
mod control_flow;
mod data_structure;
mod data_types;
mod enums;
mod functions;
mod io_demo;
mod math;
mod shadowing;

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::mem;

fn main() {
    // control_flow ------------------------------------------------------>
    // control_flow::if_statement::output();
    // control_flow::terrnary_operator::output();
    // control_flow::match_demo::output();
    // control_flow::while_demo::output();
    // control_flow::for_in::output();

    // data_types ------------------------------------------------------>
    // data_types::numbers_demo::output();
    // data_types::string_demo::output();

    // math ------------------------------------------------------>
    // math::mod_math::output();

    // io_demo ------------------------------------------------------>
    // io_demo::stdin::output();

    // shadowing ------------------------------------------------------>
    // shadowing::shadow::output();

    // data_structure ------------------------------------------------------>
    // data_structure::arrays::output();
    // data_structure::tuples::output();
    // data_structure::vectors::output();

    // casting ------------------------------------------------------>
    // casting::cast::output();

    // enunms  ------------------------------------------------------>
    // enums::enums::output();

    // functions  ------------------------------------------------------>
    functions::fns::output();
}
