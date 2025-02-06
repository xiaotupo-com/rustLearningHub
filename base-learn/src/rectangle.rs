#[derive(Default)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn get_area(&self) -> f64 {
        self.width * self.height
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }
}
