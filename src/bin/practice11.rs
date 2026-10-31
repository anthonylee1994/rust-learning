use std::f64::consts::PI;

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

trait Shape: Describe {
    fn area(&self) -> f64;
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

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
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
    let list: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(1.0, 2.0)),
        Box::new(Rectangle::new(3.0, 4.0)),
        Box::new(Circle::new(3.0)),
    ];

    for item in list.iter() {
        println!("{}", item.describe());
    }
    println!("Total area: {}", total_area(&list));
    print_area(&Rectangle::new(1.0, 2.0));
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|x| x.area()).sum()
}

fn print_area(shape: &impl Shape) {
    println!("Area: {}", shape.area())
}

// Why can you call `describe()` on a `Box<dyn Shape>`?
// `Shape: Describe` is a supertrait, so every type that implements `Shape`
// must also implement `Describe`. A `dyn Shape` therefore always has `describe()`.

// When would you pick `impl Shape` over `dyn Shape`?
// `impl Shape` fixes one concrete type at compile time, so calls are direct and fast.
// `dyn Shape` resolves the type at run time, so it needs a pointer such as `Box`,
// and it is slightly slower because of the lookup.
// Default to `impl Shape`. Use `dyn Shape` only when different types must live
// in the same collection, like `Vec<Box<dyn Shape>>`.
