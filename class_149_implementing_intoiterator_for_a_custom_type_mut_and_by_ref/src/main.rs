/*
Iterating over Car collection based on price range

Write a program featuring a custom data type named CarCollection.
This collection will store a list of Car objects, each with attributes like make, model, and price.
The key requirement is to implement the IntoIterator trait for CarCollection
in all three forms (by value, by immutable reference, and by mutable reference).
However, there's a unique twist: the next method of the iterator should iterate over
cars within a specified price range. This means when the CarCollection is iterated over,
it should only yield cars whose prices fall within a given range
(e.g., cars priced between 20,000 and 50,000 USD).
*/

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    price: u32,
}

#[derive(Debug)]
struct CarCollection {
    cars: Vec<Car>,
    price_range: (u32, u32), // Price range (min, max)
}

impl CarCollection {
    fn new(cars: Vec<Car>, price_range: (u32, u32)) -> Self {
        CarCollection { cars, price_range }
    }
}

// Custom iterator for by-value iteration
struct CarPriceRangeIteratorByValue {
    remaining_cars: std::vec::IntoIter<Car>,
    price_range: (u32, u32),
}

impl Iterator for CarPriceRangeIteratorByValue {
    type Item = Car;
    fn next(&mut self) -> Option<Self::Item> {
        //type of car is &Car
        self.remaining_cars
            .find(|car| (*car).price >= (*self).price_range.0 && car.price <= self.price_range.1)

        /*
                while let Some(car) = self.remaining_cars.next() {
                    if car.price >= self.price_range.0 && car.price <= self.price_range.1 {
                        return Some(car);
                    }
                }

                None
        */
    }
}

// Implement `IntoIterator` for `CarCollection` (by value)
impl IntoIterator for CarCollection {
    type Item = Car;
    type IntoIter = CarPriceRangeIteratorByValue;

    fn into_iter(self) -> Self::IntoIter {
        CarPriceRangeIteratorByValue {
            remaining_cars: self.cars.into_iter(),
            price_range: self.price_range,
        }
    }
}

// Custom iterator for by-ref iteration
struct CarPriceRangeIteratorByRef<'a> {
    remaining_cars: std::slice::Iter<'a, Car>,
    price_range: (u32, u32),
}

impl<'a> Iterator for CarPriceRangeIteratorByRef<'a> {
    type Item = &'a Car;
    fn next(&mut self) -> Option<Self::Item> {
        //type of car is &&Car
        self.remaining_cars
            .find(|car| car.price >= self.price_range.0 && car.price <= self.price_range.1)
    }
}

// Implement `IntoIterator` for `&CarCollection` (by immutable borrow)
impl<'a> IntoIterator for &'a CarCollection {
    type Item = &'a Car;
    type IntoIter = CarPriceRangeIteratorByRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CarPriceRangeIteratorByRef {
            remaining_cars: self.cars.iter(),
            price_range: self.price_range,
        }
    }
}

// Custom iterator for by-ref iteration
struct CarPriceRangeIteratorByMutRef<'a> {
    remaining_cars: std::slice::IterMut<'a, Car>,
    price_range: (u32, u32),
}

impl<'a> Iterator for CarPriceRangeIteratorByMutRef<'a> {
    type Item = &'a mut Car;
    fn next(&mut self) -> Option<Self::Item> {
        //type of car is &&mut Car
        self.remaining_cars
            .find(|car| car.price >= self.price_range.0 && car.price <= self.price_range.1)
    }
}

// Implement `IntoIterator` for `&mut CarCollection` (by mutable borrow)
impl<'a> IntoIterator for &'a mut CarCollection {
    type Item = &'a mut Car;
    type IntoIter = CarPriceRangeIteratorByMutRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CarPriceRangeIteratorByMutRef {
            remaining_cars: self.cars.iter_mut(),
            price_range: self.price_range,
        }
    }
}

fn main() {
    println!("\n\n--------------------\n\n");

    let cars = vec![
        Car {
            make: "Maruti Suzuki".to_string(),
            model: "Swift".to_string(),
            price: 8000,
        },
        Car {
            make: "Honda".to_string(),
            model: "City".to_string(),
            price: 12000,
        },
        Car {
            make: "Tata Motors".to_string(),
            model: "Nexon".to_string(),
            price: 10000,
        },
        // Add more cars if needed
    ];

    let car_collection_1 = CarCollection::new(cars, (8000, 13000));

    println!("iterate over car_collection by value");
    for car in car_collection_1 {
        println!(
            "Found car: {} {}, Price: {}",
            car.make, car.model, car.price
        );
    }

    println!("\n\n--------------------\n\n");

    let cars2 = vec![
        Car {
            make: "Maruti Suzuki".to_string(),
            model: "Swift".to_string(),
            price: 8000,
        },
        Car {
            make: "Honda".to_string(),
            model: "City".to_string(),
            price: 12000,
        },
        Car {
            make: "Tata Motors".to_string(),
            model: "Nexon".to_string(),
            price: 10000,
        },
        // Add more cars if needed
    ];

    let car_collection_2 = CarCollection::new(cars2, (8000, 15000));

    println!("iterate over car_collection by immutable borrow");
    for car in &car_collection_2 {
        println!(
            "Found car: {} {}, Price: {}",
            car.make, car.model, car.price
        );
    }

    println!("\n\n--------------------\n\n");

    println!(
        "here car_collection_2 still is valid: {:?}",
        car_collection_2
    );

    println!("\n\n--------------------\n\n");

    let cars3 = vec![
        Car {
            make: "Maruti Suzuki".to_string(),
            model: "Swift".to_string(),
            price: 8000,
        },
        Car {
            make: "Honda".to_string(),
            model: "City".to_string(),
            price: 12000,
        },
        Car {
            make: "Tata Motors".to_string(),
            model: "Nexon".to_string(),
            price: 10000,
        },
        // Add more cars if needed
    ];

    let mut car_collection_3 = CarCollection::new(cars3, (8000, 15000));

    println!("iterate over car_collection by mutable borrow");
    for car in &mut car_collection_3 {
        println!(
            "Found car: {} {}, Price: {}",
            car.make, car.model, car.price
        );
        car.price += 10_000;
    }

    println!("\n\n--------------------\n\n");

    println!(
        "here car_collection_3 still is valid but the prices were changed: {:?}",
        car_collection_3
    );

    println!("\n\n--------------------\n\n");
}
