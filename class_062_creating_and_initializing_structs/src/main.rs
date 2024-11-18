fn main() {
    // Creating a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Creating an instance of a struct
    let user1 = User {
        username: String::from("Danilo"),
        email: String::from("danilo_guedes_rust_dev@mail.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("user1 name: {}", user1.username);
    println!("user1 email: {}", user1.email);
    println!("user1 sign_in_count: {}", user1.sign_in_count);
    println!("user1 active: {}", user1.active);
}
