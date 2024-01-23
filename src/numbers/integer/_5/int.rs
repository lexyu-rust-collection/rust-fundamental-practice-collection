// Fix errors and panics to make it work
fn main() {
    let v1 = 251_i16 + 8;
    let v2 = u16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}
