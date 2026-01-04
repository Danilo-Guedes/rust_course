macro_rules! hello {
    () => {
        println!("Hello, world!");
    };
    ($name:literal) => {
        println!("Hello, {}!", $name);
    };
    ($name1:literal, $name2:literal) => {
        println!("Hello, {} and {}!", $name1, $name2);
    };
    ($name1:literal, $name2:literal,$($names:literal), +) => {
                println!("Hello, {} and {}!", $name1, $name2);
                println!("And also:");
        $(
            println!("Hello, {} in each line!", $names);
        )+
    };
    ($($num:expr), +) => {
        $(
            println!("Hello Number {}!", $num);
        )+
    };
}

fn main() {
    hello!();
    hello!("Danilo");
    hello!("Danilo", "Amanda");
    hello!("Danilo", "Amanda", "Giovanna");

    hello!(1, 2, 3);

    hello!((5 + 2), (5 - 2), (3 * 2));
}
