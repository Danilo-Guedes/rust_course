// Exercise-diy-8 : Check a given char array palindrome or not.
// You have to implement the is_palindrome() function

fn is_palindrome(arr: &[char; 5]) -> bool {
    // this was my implementation without see any solution
    let mut backwards_index = arr.len() - 1;

    let result = 'outer: loop {
        for letter in arr {
            println!(
                "Comparing letter '{}' with backward_letter '{}'\n",
                *letter, arr[backwards_index]
            );
            if *letter == arr[backwards_index] && backwards_index > 0 {
                backwards_index -= 1;
                continue;
            }
            break 'outer false;
        }
        break 'outer true;
    };
    result
}

fn main() {
    let char_array_1 = ['r', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_1) {
        println!("given array is a palindrome.");
    } else {
        println!("given array is not a palindrome");
    }

    let char_array_2 = ['b', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_2) {
        println!("given array is a palindrome.");
    } else {
        println!("given array is not a palindrome");
    }
}
