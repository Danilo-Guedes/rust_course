/*
Exercise-diy-13 : Implement a simple generic stack data structure
Requirements
    ==> Define a generic Stack struct that can store items of any type.
Implement the following methods for the Stack:
    ==> new: Creates a new, empty stack.
    ==> push: Adds an item to the top of the stack.
    ==> pop: Removes and returns the item from the top of the stack.
    ==> peek: Returns a reference to the top item without removing it.
    ==> is_empty: Checks if the stack is empty.
    ==> size: Returns the number of items in the stack.
    ==> clear: Removes all items from the stack.

Hints:
    ==> Define a generic struct whose name is 'Stack' generic over T
    ==> USe a 'Vector' to store the items
 */

// Define the generic Stack struct
//TODO

//Create a generic impl block
//TODO
//Note : This is partial implementation
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // Creates a new, empty stack
    fn new() -> Self {
        return Stack { items: vec![] };
    }

    // Adds an item to the top of the stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Removes and returns the item from the top of the stack
    fn pop(&mut self) -> Option<T> {
        return self.items.pop();
    }

    // Returns a reference to the top item without removing it
    fn peek(&self) -> Option<&T> {
        let index = usize::checked_sub(self.size(), 1).or(Some(0 as usize))?;
        return self.items.get(index);
    }

    // Checks if the stack is empty
    fn is_empty(&self) -> bool {
        return self.size() == 0;
    }

    // Returns the number of items in the stack
    fn size(&self) -> usize {
        return self.items.len();
    }

    // Removes all items from the stack
    fn clear(&mut self) {
        self.items.clear();
    }
}

fn main() {
    let mut stack_of_temperatures = Stack::new();

    println!("stack after creation : {:?}", stack_of_temperatures);
    stack_of_temperatures.push(33.3);
    stack_of_temperatures.push(34.9);

    println!("stack after push 2 items : {:?}", stack_of_temperatures);

    stack_of_temperatures.clear();

    println!("stack after push 2 items : {:?}", stack_of_temperatures);

    println!(
        "the stack is empty? => {:?}",
        stack_of_temperatures.is_empty()
    );
    println!("what is the size? => {:?}", stack_of_temperatures.size());

    println!(
        "this pop should return None => {:?}",
        stack_of_temperatures.pop()
    );

    println!(
        "this peek also should return None => {:?}",
        stack_of_temperatures.peek()
    );
}

#[cfg(test)]
mod test {
    use crate::Stack;

    #[test]
    fn test_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(true, stack.is_empty());
    }

    #[test]
    fn test_stack_is_not_empty() {
        let mut stack: Stack<&str> = Stack::new();
        stack.push("Rose");
        assert_eq!(false, stack.is_empty());
    }

    #[test]
    fn test_stack_clear() {
        let mut stack_of_temperatures = Stack::new();
        stack_of_temperatures.push(33.3);
        stack_of_temperatures.push(34.9);
        stack_of_temperatures.clear();
        assert_eq!(true, stack_of_temperatures.is_empty());
        assert_eq!(0, stack_of_temperatures.size());
        assert_eq!(None, stack_of_temperatures.pop());
        assert_eq!(None, stack_of_temperatures.peek());
    }

    //write more test cases.
}
