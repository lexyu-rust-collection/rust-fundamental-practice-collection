use std::ops::Add;

pub fn output() {
    println!("5 + 2 = {}", get_sum_gen(5, 2));
    println!("3.3 + 6.7 = {}", get_sum_gen(3.3, 6.7));
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
