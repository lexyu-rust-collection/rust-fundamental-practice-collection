pub fn output() {
    // say_hello();

    // get_sum(100, 50);

    println!("{}", get_sum_2(50, 5));

    println!("{}", get_sum_3(5, 5));

    let x = 2;
    let y = 10;
    println!("Before Square : {}, {}", x, y);
    println!("After Square : {:p}, {:p}", &x, &y);
    let (x, y) = get_separate_result(2, 10);
    println!("After Square : {}, {}", x, y);
    println!("After Square : {:p}, {:p}", &x, &y);
}

fn say_hello() {
    println!("Hello!!!")
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    x + y
}

fn get_separate_result(x: i32, y: i32) -> (i32, i32) {
    return (x * x, y * y);
}
