use std::{sync::Arc, thread};

fn main() {
    let mut handles = Vec::new();
    let list = Arc::new(vec![1, 2, 3]);
    println!("count = {}", Arc::strong_count(&list));

    for _ in 0..3 {
        let list = list.clone();
        let handle = thread::spawn(move || {
            println!("sum = {}", list.iter().sum::<i32>());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("count = {}", Arc::strong_count(&list));
}
