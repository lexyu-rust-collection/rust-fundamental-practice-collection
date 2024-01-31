use std::str;

pub fn output() {
    let my_string: String = String::from("Hello!");
    println!("{my_string}");

    let my_str: &str = &my_string;
    println!("{my_str}");

    let my_static_str: &'static str = "Hello, static!";
    println!("{my_static_str}");

    // --------------------------------------------------
    let mut st1 = String::new();
    st1.push('K');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
        println!("word = {}", word);
    }

    let st2 = st1.replace("K", "Another");
    println!("st2 = {}", st2);

    // --------------------------------------------------

    let st3 = String::from("a b c d e f g h i j k l a a d e c");

    let mut v1: Vec<char> = st3.chars().collect();

    v1.sort();
    println!("v1 After sort : {:?}", v1);

    v1.dedup();
    println!("v1 After remove duplicate : {:?}", v1);

    for c in v1 {
        println!("{}", c);
    }
    // --------------------------------------------------

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("st5 = {}", st5);

    let byte_arr = st5.as_bytes();
    println!("byte : {:?}", byte_arr);

    let st6 = &st5[0..5];
    println!("String length : {}", st6.len());
    st5.clear();

    println!("st5 after clear() = {}", st5);
    let st6 = String::from("Just some");

    let st7 = String::from(" words");

    let st8 = st6 + &st7;

    for ch in st8.chars() {
        print!("{} ", ch);
    }

    println!();

    for ch in st8.bytes() {
        print!("{},", ch);
    }
}
