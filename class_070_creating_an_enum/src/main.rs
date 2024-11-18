#[derive(Debug, PartialEq)]
enum CarStatus {
    MovingUp(i32, i32, i32),
    MovingDown { speed: u32 }, // could also be MovingUp(u32) if we don't need to name the field
    NotMoving(Point),          // here we are using a struct as a field,
    NotWorking,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("\n--------------------\n");
    let mut curr_car_status = CarStatus::NotMoving(Point { x: 0, y: 0 });

    println!("Current car status: {:?}", curr_car_status);

    println!("\n--------------------\n");

    curr_car_status = CarStatus::MovingUp(100, 67, 78);

    println!("Current car status: {:?}", curr_car_status);

    println!("\n--------------------\n");

    // we can use Enums to compare values if they implement the PartialEq trait

    if curr_car_status == CarStatus::NotWorking {
        println!("Car is not working");
    } else {
        println!("Car is working");
    }

    println!("\n--------------------\n");

    // we also can use patter match in some cases which is more powerful,
    // considering this pattern matches is considering the best way to handle enums compared to if else statements

    curr_car_status = CarStatus::NotMoving(Point { x: 25, y: 40 });

    match curr_car_status {
        CarStatus::MovingUp(..) => println!("Car is moving up"),
        CarStatus::NotMoving(Point { x, .. }) => println!("Car is not moving , x is : {}", x),
        _ => println!("Car is not moving up"),
    }

    println!("\n--------------------\n");
}
