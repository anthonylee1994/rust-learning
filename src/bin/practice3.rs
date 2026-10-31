fn main() {
    let name = String::from("Ferris");
    greet(&name);
    // greet(name.clone());
    println!("{}", name);
}

fn greet(n: &str) {
    println!("Hi {}", n);
}
