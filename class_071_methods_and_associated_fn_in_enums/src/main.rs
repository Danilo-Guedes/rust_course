#[derive(Debug)]
enum Shape {
    Circle { x: f32, y: f32, radius: f32 },
    Rectangle(Rectangle),
    Triangle(f32, f32, f32),
}

impl Shape {
    fn new_circle(x: f32, y: f32, radius: f32) -> Self {
        Shape::Circle { x, y, radius }
    }

    fn new_rectangle(x: f32, y: f32, h: f32, w: f32) -> Self {
        Shape::Rectangle(Rectangle::new(x, y, h, w))
    }

    fn area(&self) -> f32 {
        match self {
            Shape::Circle { radius, .. } => std::f32::consts::PI * radius * radius,
            Shape::Rectangle(rect) => rect.h * rect.w,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
}

impl Rectangle {
    fn new(x: f32, y: f32, h: f32, w: f32) -> Self {
        Rectangle { x, y, h, w }
    }
}

fn main() {
    println!("\n-------------------------\n");
    let my_circle = Shape::new_circle(0_f32, 2.0, 4.5);
    println!("my_circle: {:?}", my_circle);

    println!("Area of the my_cirle is: {}", my_circle.area());

    println!("\n-------------------------\n");

    let my_rectangle = Shape::new_rectangle(1.0, 2.0, 4.5, 5.0);

    println!("my_reclangle: {:?}", my_rectangle);

    println!("Area of the my_rectangle is: {}", my_rectangle.area());

    println!("\n-------------------------\n");

}
