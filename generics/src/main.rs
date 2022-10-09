fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(char_list);
    println!("The largest char is {}", largest);

    let _p1 = Point { x: 5, y: 10};
    let _p2 = Point { x: 1.3, y: 4.5};
    let _p3 = Point { x: 3, y: 3.5};

    let p1 = Point { x: 5, y: 10.4 };
    println!("p1.x = {}", p1.x());
    println!("p1.y = {}", p1.y());

    let p2 = Point { x: "Hello", y: 'c' };
    println!("p2.x = {}", p2.x());
    println!("p2.y = {}", p2.y());

    let p3 = Point { x: 5, y: 10 };
    let p4 = Point { x: "Hello", y: 'c' };

    let p5 = p3.mixup(p4);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);

}

// fn get_largest(list: Vec<i32>) -> i32 {
//     let mut largest = list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// struct Point {
//     x: i32,
//     y: i32,
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

