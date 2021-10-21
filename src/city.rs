use std::fmt::{Formatter, Display, Result};

#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    long: f32,
}

impl Display for City {
    fn fmt(&self,  f: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 {
            'N'
        } else {
            'S'
        };

        let long_c = if self.long >= 0.0 {
            'E'
        } else {
            'W'
        };

        write!(f, "City: {}, Location: {:.2}°{} {:.2}°{}", self.name, self.lat.abs(), lat_c, self.long.abs(), long_c)
    }
}

pub fn dispcity(nam: &'static str, la: f32, lo: f32) {
    let city = City {name: nam, lat: la, long: lo};
    println!("Display: {}", city);

    println!("Debug: {:?}", city);
    println!("Pretty Debug: {:#?}", city);
}