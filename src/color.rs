use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
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

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {red: self.red + other.red,
               blue: self.blue + other.blue,
               green: self.green + other.green}
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {red: self.red - other.red,
               blue: self.blue - other.blue,
               green: self.green - other.green}
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {red: self.red * other.red,
               blue: self.blue * other.blue,
               green: self.green * other.green}
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(self, other: Color) -> Color {
        Color {red: self.red / other.red,
               blue: self.blue / other.blue,
               green: self.green / other.green}
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {red: self.red * other,
               blue: self.blue * other,
               green: self.green * other}
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color {red: self.red / other,
               blue: self.blue / other,
               green: self.green / other}
    }
}
