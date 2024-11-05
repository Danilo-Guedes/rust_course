fn main() {

    //ARRAY HAVE FIXED SIZE, FOR DIFFERENT SIZE USE VECTORS

    let my_array = [5.7, 3.8, 9.2, 1.0, 0.5]; // Array of 5 elements [f64; 5]

    // println!("this is my array{}", my_array); // this will error because array does not implement the Display trait

    println!("this is my array {:?}", my_array); // this will print the array
    println!("this is my array {:#?}", my_array); // this will print the array in a more readable format

    //REPEAT EXPRESS IN ARRAYS

    let array2: [i32; 5] = [3; 5]; // Array of 5 elements with all elements set to 3

    println!("this is my array2 {:?}", array2); 

    //ITERATING OVER AN ARRAY

    let mut sum = 0;

    for elem in array2 {
        sum += elem;
    }

    println!("{}", sum)

}
