use std::rc::Rc;

fn main() {
    let mut data = Rc::new(50);

    println!("Initial value: {}", *data); // this is possible because Rc<T> implements Deref

    // *data += 49; // this is NOT POSSIBLE because Rc<T> does not implement DerefMut
    // println!("Updated value: {}", *data);

    if let Some(data_mut) = Rc::get_mut(&mut data) {
        *data_mut += 49; // this is possible because we have a unique reference to the Rc<T>
        println!("Updated value: {}", *data);
    } else {
        println!("Cannot get mutable reference to the data because there are multiple references.");
    }

}
