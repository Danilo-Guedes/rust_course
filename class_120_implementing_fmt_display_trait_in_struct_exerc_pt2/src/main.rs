use std::fmt;
struct Dog {
    name: String,
    age: u8,
    weight: u8,
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#^40}", "Dog details");
        println!();
        write!(f, "{:<5}{:^30}{:>5}", "Age", ":", self.age);
        println!();
        write!(f, "{:<5}{:^30}{:>5}", "Weight", ":", self.weight);
        println!();
        write!(f, "{:<5}{:^30}{:>5}", "Name", ":", self.name);
        Ok(())
    }
}

fn main() {
    println!("\n\n------------------------\n\n");

    let dog = Dog {
        name: String::from("Buddy"),
        age: 13,
        weight: 20,
    };

    println!("{}", dog);

    println!("\n\n------------------------\n\n");

}