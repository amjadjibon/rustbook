use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::error::Error;

// fn main() {
//     // println!("Hello, world!");
//     // panic!("crash and burn");
//
//     a();
//
//     let f = File::open("hello.txt");
//
//     let _f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             // panic!("There was a problem opening the file: {:?}", error)
//             match error.kind() {
//                 ErrorKind::NotFound => match File::create("hello.txt") {
//                     Ok(fc) => fc,
//                     Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
//                 },
//                 other_error => panic!("There was a problem opening the file: {:?}", other_error),
//             }
//         },
//     };
//
//
//     let _f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Tried to create file but there was a problem: {:?}", error);
//             })
//         } else {
//             panic!("There was a problem opening the file: {:?}", error);
//         }
//     });
//
//     let _f = File::open("hello.txt").unwrap();
//     let _f = File::open("hello.txt").expect("Failed to open hello.txt");
//
//     match read_username_from_file() {
//         Ok(s) => println!("username: {}", s),
//         Err(e) => println!("error: {}", e),
//     }
// }
//
// fn a() {
//     println!("a");
//     b();
//     println!("a");
// }
//
// fn b() {
//     println!("b");
//     c(5);
//     println!("b");
// }
//
// fn c(num: i32) {
//     if num == 3 {
//         panic!("c");
//     }
// }
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     // let mut s = String::new();
//     // File::open("hello.txt")?.read_to_string(&mut s)?;
//     // Ok(s)
//
//     // let mut f = File::open("hello.txt")?;
//
//     // let mut f = match f {
//         // Ok(file) => file,
//         // Err(e) => return Err(e),
//     // };
//
//     // match f.read_to_string(&mut s) {
//     //     Ok(_) => Ok(s),
//     //     Err(e) => Err(e),
//     // }
//
//     // f.read_to_string(&mut s)?;
//     // Ok(s)
//
//     fs::read_to_string("hello.txt")
// }
//
//

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
