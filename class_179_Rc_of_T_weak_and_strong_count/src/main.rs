use std::rc::Rc;

fn main() {
    let strong_rc_1 = Rc::new(String::from("Hello, Rust"));
    println!("{}", Rc::strong_count(&strong_rc_1)); // Output: 1
    println!("{}", Rc::weak_count(&strong_rc_1)); // Output: 0

    let strong_rc_2 = Rc::clone(&strong_rc_1);
    println!("{}", Rc::strong_count(&strong_rc_1)); // Output: 2
    println!("{}", Rc::weak_count(&strong_rc_1)); // Output: 0

    // create a weak reference
    let weak_ref_1 = Rc::downgrade(&strong_rc_1);
    let weak_ref_2 = weak_ref_1.clone();
    println!("{}", Rc::strong_count(&strong_rc_1)); // Output: 2
    println!("{}", Rc::weak_count(&strong_rc_1)); // Output: 2

    // upgrade weak reference to strong reference
    let strong_rc_3 = weak_ref_1.upgrade();
    println!("{}", Rc::strong_count(&strong_rc_1)); // Output: 3
    println!("{}", Rc::weak_count(&strong_rc_1)); // Output: 2

    // data access

    println!("{}", strong_rc_1);

    if let Some(upgraded_rc) = weak_ref_1.upgrade() {
        println!("Data is alive {}", upgraded_rc);
    } else {
        println!("The data has been dropped.");
    }

    // now how to mutate the data of RC<T>?

    let mut another_strong_rc = Rc::new(5);

    println!("before {}", another_strong_rc);


    // Rc::get_mut only works when there is a single strong reference
    
    if let Some(mutabe_pointer) = Rc::get_mut(&mut another_strong_rc) {
        *mutabe_pointer = 10
    }

    println!("after {}", another_strong_rc);
}
