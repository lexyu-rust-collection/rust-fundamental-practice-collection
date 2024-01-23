/*
Tips:
If we don't explicitly assign a type to a variable,
then the compiler will infer one for us.
*/

// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // Type of z ? = i32

    println!("Success!");
}
