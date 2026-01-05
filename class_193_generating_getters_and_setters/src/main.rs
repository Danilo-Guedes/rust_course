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

generate_getters!(
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

generate_setters!(
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
    let mut point = Point::new(100, 200, 300);
    let mut dimension = Dimension::new(10_f32, 20_f32);

    println!("Point: x={}, y={}, z={}", point.x, point.y, point.z);
    println!(
        "Dimension: width={}, height={}",
        dimension.width, dimension.height
    );

    println!(
        "Point BEFORE: x={}, y={}, z={}",
        point.get_x(),
        point.get_y(),
        point.get_z()
    );
    println!(
        "Dimension: width={}, height={}",
        dimension.get_width(),
        dimension.get_height()
    );

    point.set_x(101);
    point.set_y(201);
    point.set_z(301);

    dimension.set_width(11_f32);
    dimension.set_height(21_f32);

    println!(
        "Point AFTER: x={}, y={}, z={}",
        point.get_x(),
        point.get_y(),
        point.get_z()
    );

    println!(
        "Dimension: width={}, height={}",
        dimension.get_width(),
        dimension.get_height()
    );
}
