use std::str;

fn main() {
    let my_string: String = String::from("Hello!");
    println!("{my_string}");

    let my_str: &str = &my_string;
    println!("{my_str}");

    let my_static_str: &'static str = &my_str;
    println!("{my_static_str}");
}
