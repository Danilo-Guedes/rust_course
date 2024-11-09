fn main() {
    let mut num1 = 50; // mutable referent

    // let ref_of_num1 = &num1; // immutable borrow

    // *ref_of_num1 = 100; // error: cannot assign to `*ref_of_num1` because it is borrowed <<THIS WILL ERROR>>

    let _immutable_ref_of_num1 = &num1; // immutable borrow
    let _second_immutable_ref_of_num1 = &num1; // immutable borrow
    // you can create how many mutable borrows you want, but only one immutable borrow
    // if i use one of this immutable refs the compiler will complain because the rule is
    // or you create a single writer or multiple readers

    let mut_ref_of_num1 = &mut num1; // mutable borrow  --> if the num1 were not mutable, this would error

    *mut_ref_of_num1 = 100;

    println!("num1: {}", num1); // 100

    // println!("_immutable_ref_of_num1: {}", _immutable_ref_of_num1);

    fn print_the_referent_value(arg: &i32) {
        println!("The value is: {}", *arg);
    } 

    print_the_referent_value(&num1);
}
