use std::string;

pub fn output() {
    let mut str1 = String::from("Lex");
    let str3 = print_return_str(str1);
    println!("str3 = {}", str3);
}
fn print_str(x: String) {
    println!("A string {}", x);
}
fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}
fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name);
}
