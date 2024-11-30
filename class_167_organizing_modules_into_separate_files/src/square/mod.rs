
pub struct Square {
    pub side: f64,
}

impl Square {
    pub fn area(&self) -> f64 {
        self.side * self.side
    }
}
