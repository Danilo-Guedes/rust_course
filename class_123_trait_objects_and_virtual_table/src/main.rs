
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    println!("\n\n-------------------\n\n");

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
   
   let vec_of_shapes: Vec<&dyn Shape> = vec![&circle, &rectangle]; 
   // this dyn is for a technique called trait objects, which calls a dynamic dispatch
   // it stores a fat pointer, which is a pointer to the object and a pointer to the virtual table (vtable wich contains the function pointers)

   for shape in vec_of_shapes {
       print_area(shape);
   }

    println!("\n\n-------------------\n\n");
}
