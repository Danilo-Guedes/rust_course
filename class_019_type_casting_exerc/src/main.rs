// Exercise: Fix the code which causes runtime panic
// Expected output :
// Large Number 1: 4294967294
// Large Number 2: 3000000000
// Mul. Result: 1.2885e19
// First run this code and observe that it causes runtime panic
// Hints: you may have to use the 'as' keyword or/and change the data types of numbers
fn main() {
    let large_number1: u64 = 0xffff_fffe; // the answer was to change u32 to u64 in both numbers
    let large_number2: u64 = 3000000000;

    println!("Large Number 1: {}", large_number1);
    println!("Large Number 2: {}", large_number2);

    let result = large_number1 * large_number2;
    println!("Mul. Result: {}", result);
}
