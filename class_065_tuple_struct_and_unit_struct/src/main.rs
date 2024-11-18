fn main() {
    println!("\n----------------\n");

    // tuple struct

    let p = Point(10, 20.5, 30);

    println!("p.0 = {}", p.0);
    println!("p.1 = {}", p.1);
    println!("p.2 = {}", p.2);

    println!("\n----------------\n");

    // unit struct

    let nothing = do_nothing();

    println!("nothing = {:?}", nothing);

    println!("\n----------------\n");

    // passing struct by reference

    let mut danilo = User {
        name: String::from("Danilo"),
        age: 30,
    };

    println!("Danilo's age: {}", danilo.age);

    update_age(&mut danilo, 31);

    println!("Danilo's age after update fn call: {}", danilo.age);

    println!("\n----------------\n");

}

struct Point(i32, f64, u8);

#[derive(Debug)]
struct NoData;

fn do_nothing() -> NoData {
    NoData
}


struct User {
    name: String,
    age: u32,
}


fn update_age(user: &mut User, new_age: u32) {
    user.age = new_age;
    //(*user).age = new_age; ; // This is the same as the line above
}
