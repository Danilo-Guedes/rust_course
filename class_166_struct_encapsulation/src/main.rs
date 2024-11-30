mod car_module {
    pub struct Car {
        make: String,  // Private field
        model: String, // Private field
        year: u32,     // Private field
    }

    impl Car {
        // Public constructor method
        pub fn new(make: String, model: String, year: u32) -> Self {
            Car { make, model, year }
        }

        // Public getter for `make`
        pub fn get_make(&self) -> &String {
            &self.make
        }

        // Public getter for `model`
        pub fn get_model(&self) -> &String {
            &self.model
        }

        // Public method to update `year`
        // Includes validation to ensure year is within a reasonable range
        pub fn update_year(&mut self, new_year: u32) {
            if new_year > 1900 && new_year <= 2023 {
                self.year = new_year;
            }
        }
    }
}

fn main() {
    println!("\n\n---------------------------\n\n");
    let mut my_car = car_module::Car::new("Toyota".to_string(), "Camry".to_string(), 2020);

    println!("Make: {}", my_car.get_make());
    println!("Model: {}", my_car.get_model());

    my_car.update_year(2022);
    println!("Year updated!");

    println!("\n\n---------------------------\n\n");
}
