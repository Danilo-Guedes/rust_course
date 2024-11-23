use std::cmp;

fn find_max_elemet<T>(list: &[T]) -> Option<T> // this could be written like this -> fn find_max_elemet<T: cmp::PartialOrd + Copy>(list: &[T]) -> Option<T>
where
    T: cmp::PartialOrd + Copy,
{
    if list.is_empty() {
        return None;
    }

    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn main() {
    println!("\n\n-----------------------\n\n");

    let arr_int = [1, 2, 3, 4, 5];
    let arr_float = [1.1, 2.2, 3.3, 4.4, 5.5];

    println!("Max element in arr_int: {:?}", find_max_elemet(&arr_int));

    println!("\n\n-----------------------\n\n");

    println!(
        "Max element in arr_float: {:?}",
        find_max_elemet(&arr_float)
    );

    println!("\n\n-----------------------\n\n");
}
