pub fn output() {
    let num = 10;

    match num {
        1..=18 => println!("in 1 to 18"),
        25 | 50 => println!("25 or 50"),
        65..=i32::MAX => println!("in 65 to i32"),
        _ => println!("outlier"),
    }
}
