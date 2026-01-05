mod struct_utils;

make_struct_with_new!(
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    },
    struct Dimension {
        width: f32,
        height: f32,
    }
);

fn main() {
    let point = Point::new(100, 200, 300);
    let dimension = Dimension::new(10_f32, 20_f32);

    println!("Point: x={}, y={}, z={}", point.x, point.y, point.z);
    println!(
        "Dimension: width={}, height={}",
        dimension.width, dimension.height
    );
}
