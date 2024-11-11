fn main() {
    let my_array: [i32; 4] = [1, 2, 3, 4];

    let slice_1 = &my_array[1..=3]; // this is a slice of my_array and its type is &[i32] => output: [2, 3, 4]
    let slice_2 = &my_array[..]; // output: [1, 2, 3, 4]
    let slice_3 = &my_array[0..=1]; // output: [1, 2]

    //THOSE SLICES ARE IMMUTABLE, THEY CAN'T MODIFY THE ORIGINAL ARRAY
    // TO CREATE MUTABLE SLICES, USE &mut AND MY_ARRAY SHOULB BE MUTABLE AS WELL

    println!("{:?}", slice_1);
    println!("{:?}", slice_2);
    println!("{:?}", slice_3);

    //////////////////////////////////////////

    let my_array_2: [i32; 4] = [-11, 25, 34, 66];

    let slice_4 = &my_array_2[1..=3]; // this is a slice of my_array and its type is &[i32] => output: [25, 34, 66]

    let mut sum = 0;

    for i in slice_4 {
        sum += *i; // dereferencing the reference to get the value
    }

    println!("Sum of slice_4: {}", sum); // output: 125
}
