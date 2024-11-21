use std::collections::VecDeque;

fn main() {
    println!("\n\n------------------------\n\n");

    //VECDEQUE

    // VecDeque is a double-ended queue implemented with a growable ring buffer.

    // designed to be a double ended queue, it can be used as a stack, a queue, or a double-ended queue.

    // It has O(1) time complexity for push_back, push_front, pop_back, pop_front, append, and split_at_mut.

    // VecDeque is a good choice when you need a queue with fast push and pop operations.

    let mut vd = VecDeque::with_capacity(6);

    println!("vd initial: {:?}", vd);

    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);

    println!("vd after push_back: {:?}", vd);

    vd.push_front(0);
    vd.push_front(-1);
    vd.push_front(-2);

    println!("vd after push_front: {:?}", vd);

    vd.pop_back();

    println!("vd after pop_back: {:?}", vd);

    vd.pop_front();

    println!("vd after pop_front: {:?}", vd);

    println!("\n\n------------------------\n\n");

    // you can not take slice of vecdeque

    // let slice = &vd[1..3]; this will give error

    // but we can do vecdeque indexing normally

    println!("vd[1]: {}", vd[1]);

    println!("\n\n------------------------\n\n");

    // AS_SLICES

    // VecDeque can be converted to slices using as_slices method.

    let mut vd2 = VecDeque::new();

    vd2.push_back(6);
    vd2.push_back(7);
    vd2.push_back(8);

    vd2.push_front(0);
    vd2.push_front(-1);
    vd2.push_front(-2);

    let (front, back) = vd2.as_slices();

    println!("front: {:?}", front);

    println!("back: {:?}", back);

    println!("\n\n------------------------\n\n");

    // MAKE CONTIGUOUS

    let mut vd3 = VecDeque::new();

    vd3.push_back(6); //right
    vd3.push_back(7); //right
    vd3.push_back(8); //right

    vd3.push_front(0); //left
    vd3.push_front(-1); //left
    vd3.push_front(-2); //left

    vd3.make_contiguous(); // this rearranges the elements in the buffer so that the front of the deque is at index 0.

    let (front3, right3) = vd3.as_slices();

    println!("front3: {:?}", front3);

    println!("right3: {:?}", right3);

    println!("\n\n------------------------\n\n");
}
