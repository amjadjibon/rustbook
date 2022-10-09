use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    match scores.get(&team_name) {
        Some(score) => println!("{}: {}", team_name, score),
        None => println!("{}: {}", team_name, 0),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
