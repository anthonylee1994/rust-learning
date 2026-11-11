use std::thread;

fn main() {
    let mut handles = Vec::new();

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("hello from thread {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("all done");
}
