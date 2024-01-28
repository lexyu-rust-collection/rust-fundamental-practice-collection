#![allow(unused)]

pub fn output() {
    let mut num = 100;

    let can_vote = if num >= 18 { true } else { false };

    println!("{}", can_vote);
}
