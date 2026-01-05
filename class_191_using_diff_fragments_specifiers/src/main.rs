mod utils;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }
}

fn main() {
    let point = Point::new(10, 20, 30);

    show!(point.x, point.y, point.z);

    //show  show like
    // x = 10
    // y = 20
    // z = 30
}
