use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn main() {
    let mut handles = Vec::new();
    handles.push(a());
    handles.push(b());
    handles.extend(c());

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("done");
}

fn a() -> JoinHandle<()> {
    let list = Vec::from_iter(1..=100);

    thread::spawn(move || {
        let sum = list.iter().sum::<i32>();
        println!("sum = {}", sum);
    })
}

fn b() -> JoinHandle<()> {
    let a = Arc::new(1);
    thread::spawn(move || {
        let b = a.clone();
        println!("b = {}", b);
    })
}

fn c() -> Vec<JoinHandle<()>> {
    let a = Arc::new(Mutex::new(1));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let a = a.clone();
        let handle = thread::spawn(move || {
            let mut b = a.lock().unwrap();
            *b += 1;
            println!("b = {}", b);
        });
        handles.push(handle);
    }

    handles
}
