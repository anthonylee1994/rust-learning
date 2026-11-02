fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        return None;
    }

    let mut largest = items[0];
    for &item in items {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

struct Pair<T> {
    a: T,
    b: T,
}

impl<T: PartialOrd + std::fmt::Display> Pair<T> {
    fn print_larger(&self) {
        if self.a > self.b {
            println!("{} is larger", self.a);
        } else {
            println!("{} is larger", self.b);
        }
    }
}

fn main() {
    println!("{:?}", largest(&[1, 2, 3, 4, 5]).unwrap());
    println!("{:?}", largest(&[1.0, 2.0, 3.0, 4.0, 5.0]).unwrap());
    println!("{}", largest(&["apple", "banana", "cherry"]).unwrap());

    let pair = Pair { a: 1, b: 2 };
    pair.print_larger();
}
