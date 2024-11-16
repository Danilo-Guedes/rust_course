fn main() {
    let mut fruits_data: [(&str, i32, i32); 7] = [
        ("Mango-us", 50, 80),
        ("Mango-uk", 50, 80),
        ("Orange", 19, 80),
        ("Blackberry", 20, 90),
        ("Blueberry", 17, 91),
        ("Blueberry", 17, 93),
        ("Blueberry", 21, 85),
    ];

    let len = fruits_data.len();

    // THIS IS A BUBLE SORT

    //this loop is for number of passes through the list
    for _ in 0..len {
        //inner loop for the actual comparison and swapping of elements
        for i in 0..len - 1 {
            println!("elemet: {:?}", fruits_data[i]);

            if fruits_data[i] > fruits_data[i + 1] {
                fruits_data.swap(i, i + 1);
                println!("Swiped")
            }
        }

        println!("\n")
    }

    for item in fruits_data {
        println!("{:?}", item)
    }

    //TODO: print tuple array elements
}

/*

You are required to write a program to sort the (fruit-name, price, quanity) tuples by

ascending order where fruit-name is string, price and quantity are numbers.

1: Sort based on fruit-name;

2: Then sort based on price;

3: Then sort by quantiy.

The priority is that fruit-name > price > quantity.

If the following tuples are given as input to the program:

Mango-us,50,80

Mango-uk,50,80

Orange,19,80

Blackberry,20,90

Blueberry,17,91

Blueberry,17,93

Blueberry,21,85



Expected output:

("Blackberry", 20, 90)

("Blueberry", 17, 91)

("Blueberry", 17, 93)

("Blueberry", 21, 85)

("Mango-uk", 50, 80)

("Mango-us", 50, 80)

("Orange", 19, 80)



Hint :

1) Remember you can compare the tuple lexicographically.

2) Use bubble sort. logic is given in the code.

You need to compare the ith tuple with the (i+1)th tuple and swap them if necessary

3) Use swap() method for swapping array elements (https://doc.rust-lang.org/std/primitive.slice.html#method.swap)
*/
