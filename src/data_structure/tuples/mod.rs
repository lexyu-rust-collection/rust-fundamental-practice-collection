pub fn output() {
    let t: (u8, String, f64) = (100, "Lex".to_string(), 5_000_000.00);

    println!("Name :{}", t.1);

    let (v1, v2, v3) = t;

    println!("Value :{}", v1);

    let tup = (100, 50, 22, 13);
    let tup_array = [tup.0, tup.1, tup.2, tup.3];

    for i in 0..tup_array.len() {
        let x = tup_array[i];
        println!("Element at index {i}: {x}");
    }
}
