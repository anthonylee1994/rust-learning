use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let balance = Arc::new(Mutex::new(100i64));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let balance = balance.clone();
        let handle = thread::spawn(move || {
            let mut balance = balance.lock().unwrap();
            *balance += 10;
        });
        handles.push(handle);
    }

    for _ in 0..3 {
        let balance = balance.clone();
        let handle = thread::spawn(move || {
            let mut balance = balance.lock().unwrap();
            if *balance >= 20 {
                *balance -= 20;
            } else {
                println!("insufficient funds");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("balance = {}", *balance.lock().unwrap());
}
