use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let map = Arc::new(Mutex::new(HashMap::<String, usize>::new()));
    let lines = ["a b a", "b c", "a c c"];

    let mut handles = Vec::new();

    for line in lines {
        let map = map.clone();
        let handle = thread::spawn(move || {
            let mut map = map.lock().unwrap();
            for word in line.split_whitespace() {
                *map.entry(word.to_string()).or_insert(0) += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", map.lock().unwrap());
}
