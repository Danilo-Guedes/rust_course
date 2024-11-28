#[allow(dead_code)]
#[derive(Debug)]
struct Booking {
    date: String,
    guest_name: String,
    room_number: u32,
}

impl Booking {
    fn new(date: String, guest_name: String, room_number: u32) -> Self {
        Booking {
            date,
            guest_name,
            room_number,
        }
    }
}

struct BookingOnDate<'a> {
    date: &'a str,
    bookings_iter: std::slice::Iter<'a, Booking>,
}

impl<'a> BookingOnDate<'a> {
    fn new(date: &'a str, all_bookings: &'a Vec<Booking>) -> Self {
        BookingOnDate {
            date,
            bookings_iter: all_bookings.iter(),
        }
    }
}

impl<'a> Iterator for BookingOnDate<'a> {
    type Item = &'a Booking;

    fn next(&mut self) -> Option<Self::Item> {
       self.bookings_iter.find(|booking| booking.date == self.date)
    }
}

fn main() {
    println!("\n\n------------------------\n\n");

    // iter(), iter_mut(), into_iter()


//ITER
    let numbers_strings = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
    ];


    // iter() creates an iterator that yields an immutable references &T
    // which means you can read but not modify the original collection
    let iterate_by_immutable_borrow = numbers_strings[0..=1].iter(); // can create an iter from a slice


    // type of number is &String 
    for number in iterate_by_immutable_borrow {
        println!("number: {}", number);
    }

    println!("numbers_strings is still valid: {:?}", numbers_strings);



    println!("\n\n------------------------\n\n");

    //ITER_MUT

    let mut numbers_strings_2 = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
    ];


    // iter_mut() creates an iterator that yields a mutable references &mut T
    // which means you can read and modify the original collection
    let iterate_by_mutable_borrow = numbers_strings_2.iter_mut();


    // type of number is &mut String
    for number in iterate_by_mutable_borrow {
        number.push_str("-modified");
        println!("number: {}", number);
    }

    println!("numbers_strings_2 is still valid but it changed: {:?}", numbers_strings_2);

    println!("\n\n------------------------\n\n");

    //INTO_ITER

    let numbers_strings_3 = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
    ];


    // into_iter() creates an iterator that claims the ownership of the collenction type -> T

    let iterate_by_value = numbers_strings_3.into_iter();


    // type of number is String
    for number in iterate_by_value {
        println!("number: {}", number);
    }


    // THIS WOULD ERROR
    // println!("numbers_strings_3 is NOT valid bevause the ownership was moved to the iterator", numbers_strings_3);

    println!("\n\n------------------------\n\n");


    let mut all_bookings: Vec<Booking> = Vec::new();

    let booking_1 = Booking::new("2023-10-30".to_string(), "Mr.X".to_string(), 103);
    let booking_2 = Booking::new("2023-10-30".to_string(), "Mr.Y".to_string(), 193);
    let booking_3 = Booking::new("2023-10-25".to_string(), "Mr.Z".to_string(), 123);

    all_bookings.push(booking_1);
    all_bookings.push(booking_2);
    all_bookings.push(booking_3);

    let bookings = BookingOnDate::new("2023-10-30", &all_bookings);

    for booking in bookings {
        println!("{:?}", booking)
    }

    println!("\n\n------------------------\n\n");
}
