use std::fmt;
struct Dog {
    name: String,
    age: u8,
    weight: u8,
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A Dog called {}, with age: {} and weight {})", self.name, self.age, self.weight)
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
        age: 5,
        weight: 10,
    };

    println!("{}", dog);
}