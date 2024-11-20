fn main() {
    println!("\n\n---------------\n\n");

    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);

    for i in &v {
        println!("v1 -> {}", i);
    }

    println!("\n\n---------------\n\n");

    let v2 = vec![1, 2, 3, 4, 5]; // vec! is a macro that creates a new Vec<T> instance

    for i in &v2 {
        println!("V2 -> {}", i);
    }

    println!("\n\n---------------\n\n");

    // there are other ways to create a Vec<T> instance

    let arr = [1, 2, 3, 4, 5];

    let v3 = arr.to_vec();
    let v4 = Vec::from(arr);
    let v5 = Vec::from([1, 2, 3, 4]);
    let v6 = Vec::from([0; 5]);

    println!("v3 {:?}", v3);
    println!("v4 {:?}", v4);
    println!("v5 {:?}", v5);
    println!("v6 {:?}", v6);

    println!("\n\n---------------\n\n");
}
