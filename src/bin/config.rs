use rustbook::conf::config;

fn main() {
    println!("DSN: {}", config().dsn);
    println!("{}", config());
}