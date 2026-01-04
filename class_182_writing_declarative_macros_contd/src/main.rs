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
}

fn main() {
    hello!();
    hello!("Danilo");
    hello!("Danilo", "Amanda");
}
