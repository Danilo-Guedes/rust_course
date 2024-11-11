fn main() {
    let age = 10;

    if age < 18 {
        // in rust we cannot use "if age" like python
        // the body of the if statement is required
        println!("You cannot vote");
    } else {
        println!("You can vote");
    }

    // you can retrieve the result of the if statement

    let can_vote = if age < 18 {
        "You cannot vote"
    } else {
        "You can vote"
    };

    print!("{}", can_vote);

    
}
