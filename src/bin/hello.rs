use rustbook::colors;
use rustbook::city;
use rustbook::print::{hello, comment, fmtprint, fmtprint2, fmtprint3, debugprint, complexprint, points, logger_ex};
use rustbook::logs::init_logger;

fn main() {
    init_logger();
    hello();
    comment();
    fmtprint(String::from("Amjad"));
    fmtprint("Jibon".to_string());
    fmtprint2("Khan");
    fmtprint3("Hossain");
    debugprint();
    points();
    complexprint(3.5, 4.8);
    city::dispcity("Dhaka", 190.122345, -32.543216);
    colors::disprgb(255, 110, 25);
    logger_ex();
}