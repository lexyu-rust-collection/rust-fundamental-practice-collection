fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        // let y: i32 = 5; // err
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}
