fn main() {
    let s1 = String::from("hello");
    let s2 = fun1(s1); // here the ownership of s1 is moved to fun1, if we print s1 after this line, it will give error

    println!("s2: {}", s2); // now s2 has the ownership of the string

    // another exemple using arrays

    let array1 = [5, 6, 7, 8]; //type is [i32; 4] and i32 implements the Copy trait

    let array_2: [String; 3] = [
        // here strings do not implement the Copy trait
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ];

    let single_array_item = array1[1]; // this will not move the ownership of the item because i32 implements the Copy trait

    println!("single_array_item: {}", single_array_item);

    // let single_string_item = array_2[1];
    // ERROR :  cannot move out of here move occurs because `array_2[_]` has type `String`,
    // which does not implement the `Copy` trait

    let single_string_item = &array_2[1];

    println!("single_string_item: {}", single_string_item);

    // for str in array_2 {
    //     println!("str: {}", str);
    // }

    // println!("array_2: {:?}", array_2) this will give error because
    // the ownership of array_2 is moved to the for loop
    // and it is not moved back to the caller
    // we'll have to change the for loop to use references

    for str in &array_2 {
        println!("str: {}", str); // can use also *str to dereference but it is not necessary
    }

    println!("array_2: {:?}", array_2)

}

fn fun1(mut s: String) -> String {
    s.push_str(" world");
    s // here the ownership of s is moved back to the caller
}
