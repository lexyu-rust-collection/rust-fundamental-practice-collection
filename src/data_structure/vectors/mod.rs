pub fn output() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    println!("Before push = {:?}", vec2);

    vec2.push(5);

    println!("After push = {:?}", vec2);

    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd Value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec2 Length = {}", vec2.len());

    println!("Vec2 pop last element = {:?}", vec2.pop());
}
