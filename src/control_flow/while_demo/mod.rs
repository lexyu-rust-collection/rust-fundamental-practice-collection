pub fn output() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    let mut loop_idx = 0;

    while loop_idx < arr_1.len() {
        println!("Value : {}", arr_1[loop_idx]);
        loop_idx += 1;
    }
}
