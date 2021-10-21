mod print;
mod city;
mod colors;

fn main() {
    print::hello();
    print::comment();
    print::fmtprint(String::from("Amjad"));
    print::fmtprint("Jibon".to_string());
    print::fmtprint2("Khan");
    print::fmtprint3("Hossain");
    print::debugprint();
    print::points();
    print::complexprint(3.5, 4.8);
    city::dispcity("Dhaka", 190.122345, -32.543216);
    colors::disprgb(255, 110, 25)
}
