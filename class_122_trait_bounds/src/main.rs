struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 5.0, height: 10.0 };
    let rect2 = Rectangle { width: 7.5, height: 3.2 };

    println!("Area of rect1: {}", rect1.area());
    println!("Area of rect2: {}", rect2.area());
}
