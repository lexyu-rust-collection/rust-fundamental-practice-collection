// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}", 0xff);
    println!("{}", 0o77);
    println!("{}", 0b1111_1111);
    println!("{}", v);

    // assert!(v == 1579);
    assert!(v == 1597);

    println!("Success!");
}
