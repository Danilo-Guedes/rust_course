#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 35,
    };

    match person {
        Person { age: p_age @ 30, .. } => { // we can use the @ symbol to bind the value to a variable when we create a scope variable
            println!("Found 30 years old person");
        }
        Person { ref name, age: 35 } if name == "Alice" => {
            println!("Found person named {} of age 35", name);
        }
        _ => {
            println!("Found person of unknown age");
        }
    }

    println!("{:?}", person); // this will not work as person has been moved unless we use reference
}
