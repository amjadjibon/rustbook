fn main() {
    println!("Hello, world!");

    // ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    // 4. You can transfer ownership by assigning a variable to another variable.
    // 5. You can also pass ownership to a function.
    // 6. You can return ownership from a function.
    // 7. You can use references to refer to values without taking ownership of them.
    // 8. You can use references to refer to values without taking ownership of them.

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{}", s);

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x; // copy
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // move not a copy
                 // println!("{}, world!", s1); // error
    println!("{}, world!", s2); // ok

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s comes into scope
    takes_ownerships(s); // s's value moves into the function...
                         // ... and so is no longer valid here
                         // println!("{}", s); // error

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{}", x); // ok

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // tkes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_len(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_len_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // error cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);
    let r3 = &mut s; // no problem
    println!("{}", r3);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    println!("{}", r2);

    // the rules of references
    // 1. At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    let s = no_dangle(); // ok
    println!("{}", s);

    // slice type
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}", hello, world);

    let hello = first_word(&s);
    println!("{}", hello);

    let hello = first_word_slice(&s);
    println!("{}", hello);

    s.clear(); // this empties the String, making it equal to ""

    // println!("{}", hello); // error

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownerships(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_len_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // expected named lifetime parameter
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership moves out of the function and the caller is responsible for cleaning up the memory
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
