use super::Circle;

impl Circle {
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

}

pub fn arc_length(radius: f64, angle_in_degrees: f64) -> f64 {
    let angle_in_radians = angle_in_degrees.to_radians();
    radius * angle_in_radians
}
