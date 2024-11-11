fn main() {
    let mut my_array = [1, 2, 3, 4, 5];

    // let my_slice = &mut my_array;

    // my_slice.reverse();

    // you can call the reverse method directly on the array itself that the compiler will
    // automatically pass a mutable reference to the array to the reverse method

    my_array.reverse();

    println!("the array reversed {:?}", my_array);

    // find the biggest number in the array, by sorting and getting the last element

    let mut array_for_biggest = [50, 66, 3, 14, 5];

    array_for_biggest.sort();

    println!("the array sorted {:?}", array_for_biggest);

    let index = array_for_biggest.len() - 1;

    println!("The biggest number is: {}", array_for_biggest[index]);

    // using the concat method to join two arrays

    let array1 = [60, 50, 40];
    let array2 = [30, 20, 10];
    let array3 = [array1, array2].concat();

    println!("The concatenated array is: {:?}", array3);

    // using the split_at method to split an array into two arrays

    let array_to_split = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (left, right) = array_to_split.split_at(5);

    println!("The left array is: {:?} - and the right is: {:?}", left, right);

    // using the contains method to check if an array contains a specific element

    let array_to_check = [101, 102, 103, 104, 105];

    let element_to_check = 104;

    if array_to_check.contains(&element_to_check) {
        println!("The element {} is in the array", element_to_check);
    } else {
        println!("The element {} is not in the array", element_to_check);
    }
}
