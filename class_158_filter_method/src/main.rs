fn main() {
    // Example with a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect(); // into_iter() iterate by value hence only one reference is needed
    println!("Even numbers: {:?}", even_numbers);

    println!("\n\n--------------------------\n\n");

    // Example with a vector of Point structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
    ];
    let filtered_points: Vec<&Point> = points.iter().filter(|&&Point { x, y }| x > 2 && y > 2).collect(); // inter() iterate by reference hence two references are needed
    println!("Filtered points: {:?}", filtered_points);

    println!("\n\n--------------------------\n\n");

    // Example with a vector of strings
    let strings = vec!["apple", "banana", "cherry", "date"];
    let filtered_strings: Vec<&str> = strings.into_iter().filter(|s| s.contains('a')).collect();
    println!("Filtered strings: {:?}", filtered_strings);

    println!("\n\n--------------------------\n\n");
}