use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let data = Rc::new(RefCell::new(100));

    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    // mutate the data through owner1
    // borrow_mut() of the RefCell type

    *owner1.borrow_mut() += 10;

    *owner2.borrow_mut() += 20;

    println!("Final value: {}", data.borrow());

    //Summary: Why we need Interior Mutabnility in Rust ?

    // because sometimes:
    // * you have only a shared reference (&T) to a value but you still want to modify the value.
    // in safe Rust this is not allowed unless the value uses interior mutability
}
