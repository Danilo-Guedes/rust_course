fn main() {
    let s1 = String::from("Danilo");

    let letter_n = s1.chars().nth(2);

    println!("{:?}", letter_n); // this will print Some('n')

    // we can extract the character using pattern matching with if let

    if let Some(c) = letter_n {
        // c is a local variable that will hold the value of letter_n
        println!("The third letter in s1 is {}", c);
    }

    // displayin byte representation of a unicode String

    let msg = String::from("Hello+世界");

    let byte_slice : &[u8] = msg.as_bytes();

    for byte in byte_slice {
        print!("{:#X}\t", byte);
    }

    println!();

    let byte_array = msg.into_bytes();

    println!("{:?}", byte_array);
    // println!("{}", msg); // this will ERROR because msg has been moved to byte_array
}
