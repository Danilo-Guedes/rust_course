fn main() {
    println!("\n\n---------------------------\n\n");

    // SPLICE

    let mut v = vec![1, 2, 3, 4, 5];
    let new = vec![6, 7, 8, 9, 10];

    // Splice at index 2

    let spliced: Vec<_> = v.splice(2..4, new).collect(); // this will replace the range 2..4 with the new values

    println!("Spliced: {:?}", spliced);

    println!("v: {:?}", v);

    println!("\n\n---------------------------\n\n");

    //APPEND

    let mut v2 = vec![1, 2, 3, 4, 5];

    println!("v2 before: {:?}", v2);

    let mut new2 = vec![6, 7, 8, 9, 10];

    v2.append(&mut new2); // this will append the new values to the end of the vector

    println!("v2 after: {:?}", v2);

    println!("new2: {:?}", new2);

    println!("\n\n---------------------------\n\n");

    // EXTEND

    let mut v3 = vec![1, 2, 3, 4, 5];

    println!("v3 before: {:?}", v3);

    let new3 = vec![6, 7, 8, 9, 10];

    v3.extend(new3.clone()); // this will extend the vector with the new values

    println!("v3 after: {:?}", v3);

    println!("new3: {:?}", new3);

    println!("\n\n---------------------------\n\n");
}
