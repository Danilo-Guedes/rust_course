fn main() {
    println!("\n---------------------\n");

    let user_with_defaults = Person::default();
    println!("{:?}", user_with_defaults); // Person { name: "", age: 0, is_male: false, height: 0.0, initial: '\0' }

    println!("\n---------------------\n");

    // you can partially use default values

    let user = Person {
        name: "John".to_string(),
        is_male: true,
        ..Person::default()
    };

    println!("{:?}", user); //Person { name: "John", age: 0, is_male: true, height: 0.0, initial: '\0' }

    println!("\n---------------------\n");

    // update an instruct using another struct

    let user_rest = Person {
        name: "Danilo".to_string(),
        ..user // this is like the rest operator of javascript
    };

    println!("{:?}", user_rest);

    println!("\n---------------------\n");
}

#[derive(Debug, Default)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
    height: f32,
    initial: char,
}
