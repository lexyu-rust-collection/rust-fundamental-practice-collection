// Stack: Store values in a last in LIFO
// Data on the stack must have a defined fixed size
// Heap: When putting data on the heap you request a certain a
// certain amount of space. The OS finds space available and returns
// address for that space called a pointer.
// RULES
// 1. Each value has a variable that's called its owner
// 2. There is only one owner at a time
// 3. When the owner goes out of scope the value disappears
pub fn output() {
    let str1 = String::from("World");
    let str2 = str1;
    // println!("Hello {}", str1); //borrow of moved value: `str1`
}
