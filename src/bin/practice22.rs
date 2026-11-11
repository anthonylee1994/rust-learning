use std::thread;

fn main() {
    let list = Vec::from_iter(1..=100);
    let chunks = list.chunks(25);
    let mut handles = Vec::new();

    for chunk in chunks {
        let owned = chunk.to_vec();
        let handle = thread::spawn(move || owned.iter().sum::<i32>());
        handles.push(handle);
    }

    let mut total = 0;
    for handle in handles {
        let partial = handle.join().unwrap();
        println!("partial sum = {}", partial);
        total += partial;
    }

    println!("total = {total}");
}
