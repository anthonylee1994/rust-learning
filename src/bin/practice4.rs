use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Triangle { w: f64, h: f64 },
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(radius) => radius * radius * PI,
        Shape::Rect { w, h } => w * h,
        Shape::Triangle { w, h } => (w * h) / 2.0,
    }
}

fn main() {
    let circle = Shape::Circle(10.0);
    let rect = Shape::Rect { w: 10.0, h: 10.0 };
    let triangle = Shape::Triangle { w: 10.0, h: 10.0 };

    println!("Circle area: {}", area(&circle));
    println!("Rect area: {}", area(&rect));
    println!("Triangle area: {}", area(&triangle));
}
