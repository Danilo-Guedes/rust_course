#[macro_export]
macro_rules! show {
    ($($expr:expr),+ $(,)?) => {
        $(
            println!("{} = {}", stringify!($expr), $expr);
        )+
    };
}
