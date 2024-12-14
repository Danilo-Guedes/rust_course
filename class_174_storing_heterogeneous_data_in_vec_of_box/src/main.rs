trait Shape {
    fn draw(&self);
}


pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}", self.side);
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}




fn main() {
    println!("\n\n----------------------\n\n");

    let shapes : Vec<&dyn Shape> = vec![
        &Square { side: 5.0 },
        &Circle { radius: 3.0 },
    ];

    for shape in shapes {
        shape.draw();
    }

    println!("\n\n----------------------\n\n");

    // NOW USING BOX NEW

    let shapes2 : Vec<Box<dyn Shape>> = vec![
        Box::new(Square { side: 5.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];

    for shape in shapes2 {
        shape.draw();
    }

    println!("\n\n----------------------\n\n");


}
