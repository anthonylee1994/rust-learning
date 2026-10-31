struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Rectangle {
    fn describe(&self) -> String {
        format!(
            "Rectangle {{ width: {}, height: {} }}",
            self.width, self.height
        )
    }
}

impl Describe for Circle {
    fn describe(&self) -> String {
        format!("Circle {{ radius: {} }}", self.radius)
    }
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(1.0, 2.0);
    let circle = Circle::new(3.0);

    print_it(&rect);
    print_it(&circle);
}

fn print_it(item: &impl Describe) {
    println!("{}", item.describe());
}
