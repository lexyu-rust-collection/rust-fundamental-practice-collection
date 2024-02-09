use std::mem;

pub fn output() {
    let ages: [u8; 6] = [100, 20, 50, 22, 45, 67];
    println!("{:?}", ages);
    println!("Memory address of ages array: {:p}", ages.as_ptr());
    // println!("Memory address of ages array: {:p}", &ages);

    let new_ages = &ages[1..=5];
    println!("{:?}", new_ages);
    println!("Memory address of new_ages slice: {:p}", new_ages.as_ptr());
    // println!("Memory address of new_ages slice: {:p}", &new_ages);

    let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice1 = &array[5..10];
    let slice2 = &array[3..7];

    let array: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];

    let slice = &array[..]; // [ 0, 1, 2, 3, 4, 5, 6 ]
    let slice = &array[0..3]; // [ 0, 1, 2 ]
    let slice = &array[..3]; // [ 0, 1, 2 ]
    let slice = &array[2..4]; // [ 2, 3 ]
    let slice = &array[2..]; // [ 2, 3, 4, 5, 6 ]

    let mut array: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
    let array_slice = &mut array[0..5]; // [ 0, 1, 2, 3, 4 ]

    println!("{:?}", slice.len()); // 5

    for (index, item) in slice.iter().enumerate() {
        println!("index: {:?} element {:?}", index, item);
    }

    // slice[100];

    println!("{:?}", slice.get(2));
    println!("{:?}", slice.get(100)); // None

    let array: [i32; 4] = [0, 1, 2, 3];
    let array_slice = &array[..2]; // [0, 1]
    let vector = vec![1, 2, 3, 4];
    let vector_slice = &vector[0..2]; // [1, 2]
    let string = String::from("string slice");
    let string_slice = &string[0..6]; // "string"
    println!("{:?}, {:?}, {:?}", array_slice, vector_slice, string_slice);

    return_second(array_slice);
    return_second(vector_slice);
    return_second_char(string_slice);

    for c in string_slice.chars() {
        println!("{}", c)
    }

    for (i, c) in string_slice.chars().enumerate() {
        println!("{} {}", i, c)
    }

    arr_slice_mem_info();
}

fn return_second(n: &[i32]) {
    println!("{}", n[1]);
}

fn return_second_char(n: &str) {
    println!("{:?}", n.chars().nth(1));
}

fn arr_slice_mem_info() {
    let array: [i32; 500] = [0; 500];
    let slice = &array[..];
    let array_pointer = &array;
    let slice_pointer = &slice;
    let start_of_array_slice = &array[0];
    println!("--------------------------------------------");
    println!("array_pointer address: {:p}", array_pointer);
    println!("slice_pointer address: {:p}", slice_pointer);
    println!("start_of_array_slice address: {:p}", start_of_array_slice);
    println!("slice occupies {} bytes", mem::size_of_val(&slice));
    println!(
        "array_pointer occupies {} bytes",
        mem::size_of_val(&array_pointer)
    );
    println!("array occupies {} bytes", mem::size_of_val(&array));
    println!("--------------------------------------------");
}
