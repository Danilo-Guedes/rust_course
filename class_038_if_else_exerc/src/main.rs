use std::io;

// Write a program to evaluate and assign letter grades to students based on their test scores using,

// 1) if-else-if ladder statement.
// 2) match statement

// Hint:
// Assign letter grades based on these criteria:
// 90 to 100: Grade A
// 80 to 89: Grade B
// 70 to 79: Grade C
// 60 to 69: Grade D
// Below 60: Grade F

// Expected output:
// Enter student's test score: 85
// The student's grade is: B

fn main() {
    let mut input = String::new();

    println!("Enter student's test score(Valid score: 0 to 100):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let score: u32 = input.trim().parse().expect("Invalid input");

    // if-else-if ladder statement
    if score >= 90 && score <= 100 {
        println!("The student's grade is: A");
    } else if score >= 80 && score <= 89 {
        println!("The student's grade is: B");
    } else if score >= 70 && score <= 79 {
        println!("The student's grade is: C");
    } else if score >= 60 && score <= 69 {
        println!("The student's grade is: D");
    } else if score <= 59 {
        println!("The student's grade is: F");
    } else {
        println!("Invalid score");
    }

    // match statement

    let match_score = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Invalid score",
    };

    println!("The student's grade is: {}", match_score);
}
