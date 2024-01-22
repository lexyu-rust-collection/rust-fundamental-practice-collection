fn main() {
    let x: i32 = 5;
    println!("Memory address of outer x: {:p}", &x);

    {
        let x = 12;
        println!("Memory address of inner x: {:p}", &x);

        // assert_eq!(x, 5);
        assert_eq!(x, 12);
    }

    // assert_eq!(x, 12);
    assert_eq!(x, 5);

    let x = 42;
    println!("Memory address of outer x: {:p}", &x);

    println!("{}", x); // Prints "42".
}
