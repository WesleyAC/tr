pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red: red, green: green, blue: blue }
    }

    pub fn to_u8(self) -> [u8; 3] {
        [(self.red * 255.0) as u8, (self.green * 255.0) as u8, (self.blue * 255.0) as u8]
    }

    pub fn set_color(&mut self, red: f64, green: f64, blue: f64) {
        self.red = red;
        self.blue = blue;
        self.green = green;
    }
}
