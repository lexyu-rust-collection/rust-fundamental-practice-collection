use std::thread;
use std::time::Duration;

pub fn output() {
    let t1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..21 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    t1.join().unwrap();
}

pub fn output2() {
    let mut bank = Bank { balance: 100.0 };
    withdraw(&mut bank, 35.0);
    println!("{:#?}", bank);

    /*
    to force the closure to take ownership of `bank`
    (and any other referenced variables), use the `move` keyword: `move
     */
    // thread::spawn(|| customer(&mut bank)).join().unwrap(); // need smart pointer
}

#[derive(Debug)]
struct Bank {
    balance: f32,
}

fn withdraw(the_bank: &mut Bank, amount: f32) {
    the_bank.balance -= amount;
}

fn customer(the_bank: &mut Bank) {
    withdraw(the_bank, 10.00);
}
