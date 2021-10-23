pub fn hello() {
    println!("Hello, world!");
}

pub fn comment() {
    // this is comment

    /*
    this
    is
    also
    comment
    */

    println!("Good Luck!")
}

pub fn fmtprint(name: String) {
    println!("Hi, My name is {0}", name)
}

pub fn fmtprint2<'a>(name: &'a str) {
    println!("Hi, My name is {0}", name)
}

pub fn fmtprint3(name: &str) {
    println!("Hi, My name is {0}", name)
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

pub fn debugprint() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}

// ------------------------

use std::fmt;

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

pub fn points() {
    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Prety Debug: {:#?}", point);
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imaginary: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Complex Number: {} + {}i", self.real, self.imaginary)
    }
}

pub fn complexprint(re: f32, im: f32) {
    let num = Complex{ real: re, imaginary: im };
    println!("Display: {}", num);
    println!("Debug: {:?}", num);
    println!("Pretty Debug: {:#?}", num);
}

use log::{LevelFilter, debug, error, info, warn};
pub fn logger_ex() {
    info!("this is {}", LevelFilter::Info);
    error!("this is {}", LevelFilter::Error);
    debug!("this is {}", LevelFilter::Debug);
    warn!("this is {}", LevelFilter::Warn);
}
