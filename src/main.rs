#![allow(unused)]

mod casting;
mod closures;
mod control_flow;
mod data_structure;
mod data_types;
mod enums;
mod envs;
mod error_handling;
mod functions;
mod generic;
mod io_demo;
mod iterators;
mod macros;
mod math;
mod modules;
mod ownership;
mod shadowing;
mod smart_pointer;
mod structs;
mod threads;
mod traits;

use crate::modules::order_food;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // control_flow ------------------------------------------------------>
    // control_flow::if_statement::output();
    // control_flow::terrnary_operator::output();
    // control_flow::match_demo::output();
    // control_flow::while_demo::output();
    // control_flow::for_in::output();

    // casting ------------------------------------------------------>
    // casting::cast::output();

    // closures ------------------------------------------------------->
    // closures::output();

    // data_types ------------------------------------------------------>
    // data_types::numbers_demo::output();
    // data_types::string_demo::output();

    // data_structure ------------------------------------------------------>
    // data_structure::arrays::output();
    // data_structure::tuples::output();
    // data_structure::vectors::output();
    // data_structure::hashmaps::output();

    // error_handling ------------------------------------------------------>
    // error_handling::result::output();

    // envs ------------------------------------------------------>
    // envs::env_toml_path();

    // macros ----------------------------------------------------------->
    // macros::declarative::output();
    macros::eg1::output();

    // math ------------------------------------------------------>
    // math::mod_math::output();

    // io_demo ------------------------------------------------------>
    // io_demo::stdin::output();

    // iterators ------------------------------------------------->
    // iterators::output();

    // shadowing ------------------------------------------------------>
    // shadowing::shadow::output();

    // enunms  ------------------------------------------------------>
    // enums::enums::output();

    // functions  ------------------------------------------------------>
    // functions::fns::output();

    // generic  ------------------------------------------------------>
    // generic::generic::output();

    // ownership  ------------------------------------------------------>
    // modules::order_food();
    // order_food();
    // ownership::move_eg::output();

    // ownership  ------------------------------------------------------>
    // ownership::eg4::output();

    // structs  ------------------------------------------------------>
    // structs::structs::output();

    // smart_pointer ------------------------------------------------------>
    // smart_pointer::boxes::output();
    // smart_pointer::rcs::output();

    // threads ------------------------------------------------------>
    // threads::concurrency::output2();

    // traits  ------------------------------------------------------>
    // traits::traits::output();
}

pub fn output() {}
