fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("x = {}", x);

    // this will be error because by default rust variable is immutable
    // x = 6;
    // println("x = {}", x);

    let mut y = 5;
    println!("y = {}", y);
    // this will not be an error
    y =  8;
    println!("y = {}", y);

    // constants are immutable
    // you can not make mutable constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // this is will be error
    // const mut CAN_NOT_RUN = 123;

    const MILLIONS: u32 = 1_000_000;
    println!("Millions = {}", MILLIONS);

    // shadowing

    let a = 5;
    println!("a = {}", a);

    let a = "five";
    println!("a = {}", a);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // data types
    // scalar data types
    // 1. integer
    // 2. floating point
    // 3. boolean
    // 4. char

    // integer
    let a = 76;
    let b = 0xabc;
    let c = 0o77;
    let d = 0b1010101;
    let e = b'a';
    let f: u8 = 255;

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}", a, b, c ,d ,e, f);

    let f = 3.14;
    let g: f32 = 23.34;

    println!("f = {}, g = {}", f, g);

    println!("f/g = {}", f/g);
    println!("f*g = {}", f*g);

    let t = true;
    let f = false;

    println!("true = {}, false = {}, true&false = {}", t, f, t&f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c = {}, z = {}, cat = {}", c, z, heart_eyed_cat);

    // compound types
    // the tuple types
    let tup: (i32, f64, u8) = (300, 3.14, 1);
    let (x, y, z) = tup;

    println!("x = {x}, y = {y}, z = {z}");

    let tup2 = (500, 10.3, 3);
    println!("x = {}, y = {}, z = {}", tup2.0, tup2.1, tup2.2);

    // array types
    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);

    another_function();

    println!("{}", plus_one(1));

    let x = {
        let y = 3;
        y + 1
    };

    println!("x = {}", x);

    // conditional flow
        let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    };

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loop
    let mut counter = 0;
    let result = loop {
        println!("again");
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("result = {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn another_function() {
    println!("this is another_function");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

