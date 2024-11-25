use std::fmt;
struct Dog {
    name: String,
    age: u8,
    weight: u8,
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(w) = f.width() { // we need to use the w$ sign to specify we're using the variable w
            write!(f, "{:#^w$}", "Dog details", w = 2 * w); // here we can manipulate the width of the output
            println!();
            write!(f, "{:<w$}:{:>w$}", "Age", self.age);
            println!();
            write!(f, "{:<w$}:{:>w$}", "Weight", self.weight);
            println!();
            write!(f, "{:<w$}:{:>w$}", "Name", self.name);
        } else {
            write!(f, "{:#^40}", "Dog details");
            println!();
            write!(f, "{:<20}:{:>20}", "Age", self.age);
            println!();
            write!(f, "{:<20}:{:>20}", "Weight", self.weight);
            println!();
            write!(f, "{:<20}:{:>20}", "Name", self.name);
        }
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

    println!("{:60}", dog); // this :60 is the width of the output

    println!("\n\n------------------------\n\n");
}
