use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn output() {
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));

    let handles = (0..10).map(|_| {
        let bank_ref: Arc<Mutex<Bank>> = bank.clone();
        thread::spawn(move || customer(&bank_ref))
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}

#[derive(Debug)]
struct Bank {
    balance: f32,
}

fn withdraw(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
    let mut bank_ref = the_bank.lock().unwrap();
    if bank_ref.balance < 5.00 {
        println!(
            "Current Balance : {} Withdrawal a smaller amount",
            bank_ref.balance
        );
    } else {
        bank_ref.balance -= amount;
        println!(
            "Customer withdraw {} Current Balance {}",
            amount, bank_ref.balance
        );
    }
}

fn customer(the_bank: &Arc<Mutex<Bank>>) {
    withdraw(&the_bank, 5.00);
}
