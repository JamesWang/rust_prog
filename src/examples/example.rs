use std::borrow::Cow;
use std::iter::Sum;

fn ex1() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}

#[test]
fn test_ex1() {
    ex1();
}


pub struct Point<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Point<T> {
    pub(crate) fn x(&self) -> &T {
        &self.x
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//#[test]
pub fn tweet_test() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())
}

//trait bound
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}


fn remove_spaces(input: &str) -> Cow<str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    return Cow::Borrowed(input);
}

fn remove_spaces2(input: &str) -> Cow<str> {
    if input.contains(' ') {
        input.chars()
            .filter(|&x| x != ' ')
            .collect::<std::string::String>()
            .into()
    } else {
        input.into()
    }
}

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2{ x: 0.0, y: 0.0};
    const UNIT: Vector2 = Vector2{ x: 1.0, y: 0.0};
}

pub fn ex_struct() {
    //let scaled = Vector2::UNIT.scaled_by(2.0);
}

//given any specific lifetime 'elt, you can make an Extrama<'elt> that holdsreferences with that lifetime
struct Extrama<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extram<'s>(slice: &'s [i32]) -> Extrama<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrama { greatest, least }
}

#[test]
fn test_remove_spaces() {
    let s = remove_spaces("Herman Radtke");
    println!("Length of string is {}", s.len());
}