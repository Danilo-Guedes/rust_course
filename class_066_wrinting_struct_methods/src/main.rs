#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // this method borrow the struct immutably
    fn distance_from_origin(&self) -> f32 {
        // could also be written as fn distance_from_origin(self: &Point) -> f32
        let sum_of_squares = self.x.powi(2) + self.y.powi(2);
        sum_of_squares.sqrt()
    }

    // this method borrow the struct mutably
    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    // this method takes ownership of the struct
    fn into_tuple(self) -> (f32, f32) {
        // notice that has no & sign
        (self.x, self.y)
    }
}

fn main() {
    println!("\n---------------------\n");
    let mut p = Point { x: 3.0, y: 4.0 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
    println!("p.distance_from_origin() = {}", p.distance_from_origin());

    println!("\n---------------------\n");

    p.translate(1.0, 2.0);

    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);

    println!("\n---------------------\n");

    let points = p.into_tuple();

    println!("Points as  tuples : {:?}", points);

    println!("old p: {:?}", p); // this will not work because p has been moved to points, 
    // but if Points implemneted Copy trait, it will work because it will be copied to points

    println!("\n---------------------\n");



}
