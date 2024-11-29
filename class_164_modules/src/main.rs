fn log_vehicle_details(make: &str, model: &str, year: u32) {
    println!("Make: {}, Model: {}, Year: {}", make, model, year);
}

mod cars {
    pub struct Car {
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(car: &Car) {
        crate::log_vehicle_details(&car.make, &car.model, car.year); // crate means we're acessing the root module
    }
}

mod motorcycles {
    pub struct Motorcycle {
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(motorcycle: &Motorcycle) {
        crate::log_vehicle_details(&motorcycle.make, &motorcycle.model, motorcycle.year);
    }
}

fn main() {
    println!("\n\n-----------------------------\n\n");
    let my_car = cars::Car {
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2020,
    };

    let my_motorcycle = motorcycles::Motorcycle {
        make: "Honda".to_string(),
        model: "CBR".to_string(),
        year: 2018,
    };

    cars::info(&my_car);
    motorcycles::info(&my_motorcycle);

    println!("\n\n-----------------------------\n\n");
}
