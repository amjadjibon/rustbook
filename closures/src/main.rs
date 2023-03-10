fn main() {
    let outer_var = 42;

    let closure = |x| {
        println!("outer_var: {}", x);
    };

    closure(outer_var);

    let mut counter = 0;
    let mut inc = || {
        counter += 1;
        println!("counter: {}", counter);
    };

    inc();

    let vec = vec![1, 2, 3];
    vec.into_iter()
        .for_each(|x| println!("{}", x));

    let vec2 = vec![1, 2, 3]
        .into_iter()
        .map(|x| x + 1)
        .collect::<Vec<_>>();

    println!("{:?}", vec2);

    let vec3 = vec![1, 2, 3];
    let find = vec3.into_iter().find(|&x| x == 5);

    match find {
        Some(x) => println!("Found: {}", x),
        None => println!("Not found"),
    }

    let x = 5;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("Can y be equal to x? {}", equal_to_x(y));

    let mut x = 0;
    let mut increment = || {
        x += 1;
        println!("x is now {}", x);
    };
    increment();
    increment();
    increment();
}
