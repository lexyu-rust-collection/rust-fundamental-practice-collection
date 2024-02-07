#[macro_export]
macro_rules! myvec {
    ($($x:expr), *) => {{
        let mut temp_vec = Vec::new();
		$(
			temp_vec.push($x);
		)*
		temp_vec
    }};
}
#[macro_export]
macro_rules! hey {
    ($name:expr) => {
        println!("Hey {}", $name);
    };
}

pub fn output() {
    hey!("Stranger!");

    let v1 = myvec![1, 2, 3];
    let v2 = myvec!["a", "b", "c", "d", "e"];

    println!("{:#?}", v1);
    println!("{:#?}", v2);

    for val in v1.iter() {
        print!("{val} ");
    }
    println!();
    for val in v2.iter() {
        print!("{val} ");
    }
}
