fn main() {
    //VARIABLES LEARINING

    // type inference

    let value: u16 = 10;

    let sum = 10 + value;

    println!("Sum is: {}", sum);

    // WRONG TYPE

    let value1: i32 = 10;
    let value2: i32 = 20;

    // let sum1: u8 = value1 + value2;

    // println!("Sum is with wrong type: {}", sum); // this will not run

    //TYPE CASTING

    let sum2: u8 = (value1 + value2) as u8;

    println!("Sum is with type casting: {}", sum); // this will run

    // SMART TYPE INFERENCE
    // the rust compiler will infer the type of the variable based on the operations that are being performed on it

    let value3 = 15;
    let value4 = 30;

    let sum3: u8 = value3 + value4;

    println!("type of value3: {}", type_of(&value3));
    println!("type of value4: {}", type_of(&value4));

    println!("Here the rust compiler tranform value3 and value4 in u8: {}", sum3);

    // MULTIPLYING FLOAT AND INTEGER TO SEE HOW THE COMPILER WILL BEHAVIOUR

    let value5 = 50;
    let value6 = 12.3;

    let result = value5 * value6; // we can cast value6 to i32 to avoid the error [ value5 * (value6 as i32)]

    println!("result of multiplication is {}", result); // this will error (no implementation for `{integer} * {float}` in Mul trait)

    //
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
