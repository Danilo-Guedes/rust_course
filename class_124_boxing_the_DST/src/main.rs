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

fn print_area(shape: Box<dyn Shape>) {
    println!("Area: {}", shape.area()); // here we could use *(shape).area() to dereference the Box but it is not necessary, the compiler will do it for us
}

fn main() {
    println!("\n\n-------------------\n\n");

    // BOX is a smart pointer that points to heap allocated memory rather than stack allocated memory.
    // is called smart pointer because can automatically handles the allocation and deallocation of heap memory.

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    let vec_of_shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rectangle)];
    // this dyn is called trait objects, this is also a DST (dynamically sizes type), which allow to use dynamic dispatch,
    // where the function to be called is determined at runtime
    // it stores a fat pointer, which is a pointer to the object data (inicialization and properties) and a pointer to the virtual table (vtable wich contains the function pointers)

    for shape in vec_of_shapes {
        print_area(shape);
    }

    println!("\n\n-------------------\n\n");

    // println!("Circle radius: {}", circle.radius);  // this will ERROR because circle has been moved to the vec_of_shapes
}
