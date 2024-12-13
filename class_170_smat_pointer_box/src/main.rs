fn main() {
    println!("\n\n---------------------------\n\n");

    /*
    In Rust we have several smart pointers, which are data structures that act like pointers
    but have additional metadata and capabilities. some of them are:
    - Box<T> for allocating values on the heap
    - Rc<T>, a reference counting type that enables multiple ownership
    - Arc<T>, an atomically reference counted type
    - Cell<T>, a type that provides interior mutability, even in a non-concurrent context
    - RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
    - Mutex<T>, a type that provides interior mutability, often used for mutability in concurrent code
    - RwLock<T>, a reader-writer lock
    - Cow<T>, for "clone on write" smart pointers
     */

    // Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

    let p: Box<i32> = Box::new(100);

    println!("the value of p is ({}), and it's stored in {:p}", p, p);

    println!("\n\n---------------------------\n\n");

    // they can be muttable

    let p_old = 10;

    let mut p2: Box<i32> = Box::new(p_old);

    *p2 = 20;

    println!(
        "the old value of p2 was ({}), the new value of p2 is ({}), and it's stored in {:p}",
        p_old, p2, p2
    );

    println!("\n\n---------------------------\n\n");

    // BOX OF STRING

    let s = "Hey Danilo how are you?".to_string();

    let s_box = Box::new(s);

    println!(
        "the value of s_box is ({}), and it's stored in {:p}",
        s_box, s_box
    );
}
