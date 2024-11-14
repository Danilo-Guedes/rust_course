fn main() {
    // call by value is when you pass a value to a function and the function receives a copy of the value
    // call by reference is when you pass a reference to a function and the function receives a reference to the value
    let arr = [10, 20, 30, 40, 50];

    let max_value_by_value = find_greatest_values_by_value(arr);
    println!("Max value by value: {}", max_value_by_value);

    println!("Array after call by value: {:?}", arr);

    let max_value_by_reference = find_greatest_values_by_reference(&arr);
    println!("Max value by reference: {}", max_value_by_reference);

    println!("Array after call by reference: {:?}", arr);

    // you can create how many references for the samue value you want

    let ref1 = &arr;
    let ref2 = &ref1;
    let ref3 = &ref2;

    println!("ref1: {:?}", *ref1); // type is &[i32; 5], we can also use dereference operator to get the value using the * operator
    println!("ref2: {:?}", ref2); // type is &&[i32; 5]
    println!("ref3: {:?}", ***ref3); // type is &&&[i32; 5]
}

fn find_greatest_values_by_value(arr: [i32; 5]) -> i32 {
    let mut max = arr[0];
    for item in arr.iter() {
        if *item > max {
            max = *item;
        }
    }
    max
}

fn find_greatest_values_by_reference(arr: &[i32; 5]) -> i32 {
    let mut max = arr[0];
    for item in arr.iter() {
        if *item > max {
            max = *item;
        }
    }
    return max;
}
