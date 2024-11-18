fn main() {
    // we can initialize structs using a shorthand if the variable name and the field name are the same

    let name = String::from("Alice");
    let age = 30;

    let alice = Person { name, age };

    println!("{} is {} years old", alice.name, alice.age);

    println!("\n--------------------------------\n");

    // structs can be printed using the Debug trait

    println!("{:?}", alice);

    println!("\n--------------------------------\n");


    // once you point a property of a struct to another variable, depending on the type of the property, the struct will be moved or copied

    let _alice_name = alice.name;

    // println!("{:?}", alice); // THIS WILL NOT COMPILE, BECAUSE alice.name WAS MOVED TO alice_name

    // but we can use other variables that were not moved

    println!("She is {} years old", alice.age);

    println!("\n--------------------------------\n");

    // what about the struct itself? is it copied or moved?

    let p1 = Point { x: 10, y: 20 };

    let _p2 = p1;

    // println!("{:?}", p1); // THIS WILL NOT COMPILE, BECAUSE p1 WAS MOVED TO p2

    // of the strucv implements the Copy trait, it will be copied instead of moved

    let p3 = PointWithCopy { x: 10, y: 20 };

    let p4 = p3;

    println!("p3: {:?} \n p4: {:?}", p3, p4); // THIS WILL COMPILE, BECAUSE p3 WAS COPIED TO p4

    println!("\n--------------------------------\n");

    // if we want to copy a struct that does not implement the Copy trait, we can use the clone method

    let p5 = PersonWithClone { name: String::from("Bob"), age: 40 };

    let p6 = p5.clone();

    println!("p5: {:?} \n p6: {:?}", p5, p6); // THIS WILL COMPILE, BECAUSE p5 WAS COPIED TO p6

}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)] // notice that this is possible because the struct only contains primitive types that implement the Copy trait
struct PointWithCopy {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct PersonWithClone {
    name: String,
    age: u8,
}
