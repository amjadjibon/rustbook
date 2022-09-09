#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("jon@gmail.com"),
        username: String::from("jon"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {:?}", user1);

    let name = user1.username;
    println!("name: {}", name);

    user1.username = String::from("jonathan");

    println!("user1: {:?}", user1);

    let user2 = build_user(String::from("toby"), String::from("boby@gmail.com"));
    println!("user2: {:?}", user2);

    let user3 = User {
        email: String::from("new"),
        username: String::from("new"),
        ..user1
    };

    println!("user3: {:?}", user3);
    println!("user1: {}", user1.email);
    println!("user1: {}", user1.username);
    println!("user1: {}", user1.active);
    println!("user1: {}", user1.sign_in_count);

    let height = 10;
    let width = 20;

    println!("area: {}", area(height, width));

    let rect = (10, 20);
    println!("area: {}", area_tuple(rect));

    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("rect: {:#?}", rect);
    println!("area: {}", area_struct(&rect));

    println!("area: {}", rect.area());

    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };

    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(10);
    println!("rect3: {:#?}", rect3);
    println!("area: {}", rect3.area());

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

