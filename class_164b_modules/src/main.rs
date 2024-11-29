mod parent {
    fn parent_private_function() {
        println!("I'm private to the parent module");
    }

    pub fn parent_public_function() {
        println!("I'm public in the parent module");
    }

    // pub fn access_child_private_function() { // THIS IS NOT ALLOWED
    //     child::child_private_function();
    // }

    pub mod child {
        pub fn child_public_function() {
            // Call private function of parent
            super::parent_private_function(); // OK
        }

        fn child_private_function() {
            println!("I'm child fn and not public");
        }
    }
}

fn main() {
    println!("\n\n--------------------------------\n\n");
    println!("We can access parent's public function");
    parent::parent_public_function(); // Access public function in parent

    println!("\n\n--------------------------------\n\n");
    println!("We can access child's public function");
    // Access public function in child
    parent::child::child_public_function();

    // Access child private function via parent's public interface
    // parent::access_child_private_function();         // THIS IS NOT ALLOWED
}
