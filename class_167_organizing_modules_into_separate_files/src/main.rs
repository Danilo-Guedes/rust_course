// mod circle { // THIS MOD CAN LIVE IN A SEPARATE FILE CALLED CIRCLE.RS OR CIRCLE/MOD.RS
//     pub struct Circle{
//         pub radius: f64,
//     }

//     impl Circle {
//         pub fn area(&self) -> f64 {
//             std::f64::consts::PI * self.radius * self.radius
//         }
//     }

// }

// mod square {
//     pub struct Square {
//         pub side: f64,
//     }

//     impl Square {
//         pub fn area(&self) -> f64 {
//             self.side * self.side
//         }
//     }
// }

/*These statements instruct the Rust compiler to look for and include the contents of specific files.
* `mod circle;` The compiler will look for a file named `circle.rs`
* (or a `circle/mod.rs` file if you're using a directory for the module).
* This file should be in the same directory as the file containing the `mod circle;` statement.
* The same applies to `mod square;`.
*/

mod circle;
mod square;

fn main() {
    println!("\n\n------------------------------\n\n");

    let c = circle::Circle { radius: 5.0 };
    println!("Area of the circle: {:.2}", c.area());
    println!("Circumference of the circle: {:.2}", c.circumference());

    let angle = 90.0;
    let arc_length = circle::properties::arc_length(c.radius, angle);

    println!("Arc length of the circle: {:.2}", arc_length);

    let s = square::Square { side: 4.0 };
    println!("Area of the square: {}", s.area());

    println!("\n\n------------------------------\n\n");
}
