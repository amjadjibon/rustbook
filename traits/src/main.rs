use core::fmt::{Debug, Display};

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
   // fn summarize(&self) -> String {
   //     format!("{}, by {} ({})", self.headline, self.author, self.location)
   // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

fn notify4<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// fn notify5(item: &(impl Summary + Display)) {
//     println!("Breaking news! {}", item.summarize());
// }
//
// fn notify6<T: Summary + Display>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }


fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl <T: Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify(&article);

    notify2(&tweet, &article);
    notify4(&tweet, &article);

    println!("{}", return_summarizable().summarize());

    let pair = Pair::new(1, 2);
    pair.cmp_display();
    println!("{}", pair.to_string());
}
