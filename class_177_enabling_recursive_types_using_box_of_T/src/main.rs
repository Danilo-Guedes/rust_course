#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let node_4 = Node {
        data: 4,
        next: None,
    };

    let node_3 = Node {
        data: 3,
        next: Some(Box::new(node_4)),
    };

    println!("{:#?}", node_3);

    // Box<T> provides a fixed-size pointer to the data, regardless of the actuall data's size.
}
