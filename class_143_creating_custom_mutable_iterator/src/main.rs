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

#[allow(dead_code)]
struct BookingOnDate<'a> {
    date: &'a str,
    bookings_iter: std::slice::Iter<'a, Booking>,
}

impl<'a> BookingOnDate<'a> {
    fn _new(date: &'a str, all_bookings: &'a Vec<Booking>) -> Self {
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

#[allow(dead_code)]
struct BookingOnDateMut<'a> {
    date: &'a str,
    bookings_iter: std::slice::IterMut<'a, Booking>,
}

impl<'a> BookingOnDateMut<'a> {
    fn new(date: &'a str, all_bookings: &'a mut Vec<Booking>) -> Self {
        BookingOnDateMut {
            date,
            bookings_iter: all_bookings.iter_mut(),
        }
    }
}

impl<'a> Iterator for BookingOnDateMut<'a> {
    type Item = &'a mut Booking;

    fn next(&mut self) -> Option<Self::Item> {
       self.bookings_iter.find(|booking| booking.date == self.date)
    }
}

fn main() {
    println!("\n\n------------------------\n\n");

    // ADDING A MUT ITERATOR


    let mut all_bookings: Vec<Booking> = Vec::new();

    let booking_1 = Booking::new("2023-10-30".to_string(), "Mr.X".to_string(), 103);
    let booking_2 = Booking::new("2023-10-30".to_string(), "Mr.Y".to_string(), 193);
    let booking_3 = Booking::new("2023-10-25".to_string(), "Mr.Z".to_string(), 123);

    all_bookings.push(booking_1);
    all_bookings.push(booking_2);
    all_bookings.push(booking_3);

    let bookings = BookingOnDateMut::new("2023-10-30", &mut all_bookings);

    for booking in bookings {
        booking.room_number = 666; // NOW WE CAN MUTATE ITS DATA
        println!("{:?}", booking)
    }

    println!("\n\n------------------------\n\n");
}
