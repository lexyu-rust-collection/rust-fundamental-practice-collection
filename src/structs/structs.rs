pub fn output() {
    let mut bobby = Customer {
        name: String::from("Bob Smith"),
        address: String::from("111 Main St."),
        balance: 111111.50,
    };

    bobby.address = String::from("212 Main St.");

    println!("{:?}", bobby);

    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };

    println!("{:?}", rec);
}

#[derive(Debug)]
struct Customer {
    name: String,
    address: String,
    balance: f32,
}
#[derive(Debug)]
struct Rectangle<T, U> {
    length: T,
    height: U,
}
