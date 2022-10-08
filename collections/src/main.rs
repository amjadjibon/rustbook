fn main() {
    println!("Hello, world!");

    let a = [1, 2, 3, 4, 5];

    // let v: Vec<i32> = a.iter().map(|x| x + 1).collect();
    // println!("{:?}", v);

    let mut v:Vec<i32> = Vec::new();
    for i in a.iter() {
        v.push(i + 1);
    }

    println!("{:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];
    println!("{:?}", v2);

    let third = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    match v2.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v3 = vec![1, 2, 3, 4, 5];
    let third = &v3[2];
    v.push(6);

    println!("The third element is {}", third);

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }

    // strings are utf-8 encoded bytes
    let mut s = String::new();
    s.push_str("foo");
    s.push_str("bar");
    s.push('!');

    println!("{}", s);

    let s1 = String::from("foo");
    let s2 = String::from("bar");

    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}", s4, s5, s6);

    println!("{}", s7);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


}
