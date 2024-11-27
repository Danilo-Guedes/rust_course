fn main() {
    println!("\n\n-------------------------\n\n");
    
    let y = 1;

    // even though this 2 clore are indentical, by the fact they capture the enviroment
    // the compiler generate differente types for them, so they can't be stored in the same vector
    // that's why we need to use Box<dyn Fn(i32) -> i32> to store them in the same vector

    let c1: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + y); // the type is not need here, but it's just to show the type
    let c2 = Box::new(|x| x + y);

    let vec_of_closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![c1, c2];

    println!("{}", vec_of_closures[0](1));


    println!("\n\n-------------------------\n\n");

}