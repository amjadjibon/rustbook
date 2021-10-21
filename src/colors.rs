use std::fmt::{Formatter, Display, Result};

#[derive(Debug)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.red, self.green, self.blue)
    }
}

pub fn disprgb(r: u8, g: u8, b: u8) {
    let color = RGB { red: r, green: g, blue: b};
    println!("RGB: {}", color)
}