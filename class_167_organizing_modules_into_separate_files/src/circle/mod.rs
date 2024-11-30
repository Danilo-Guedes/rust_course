pub mod properties; // this need to be public so  main.rs file can access public functions in this module

//we could also reexport the acr_length function from the properties module
// to encapsulate and hide the properties module from the main.rs file

// pub use properties::arc_length;

// the usage would be like this in the main.rs file

// let arc_length = circle::arc_length(c.radius, angle);

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
