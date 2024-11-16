fn main() {
    let date_tuple = (
        "Monday".to_string(), // string does not implement Copy trait hence it will be moved
        "25".to_string(),
        "January".to_string(),
        "2024".to_string(),
    );

    match date_tuple {
        (day, ..) if day == "Tuesday" => println!("It's Tuesday!"),
        _ => println!("It's not Tuesday!"),
    }

   // println!("date_tuple: {:?}", date_tuple); // this will not compile, because date_tuple has been partially moved
}
