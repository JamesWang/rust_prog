use std::env;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
    }

    #[test]
    fn test_gcd2() {
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n)
        }
        m %= n;
    }
    n
}

pub fn gcd_runner() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"))
    }

    if numbers.is_empty() {
        eprint!("Usage: gcd Number ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    /*
        &numbers to tell Rush that ownership of the vector(numbers) should remain
        with numbers; just borrowing its element for the loop.
        & - borrows a reference to the vector's elements
        * - *m, dereference m, yielding the value it refers to

        numbers own the vector, when numbers goes out of scope, Rust will automatically frees it
    */
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    //associated functions, functions not defined as an impl block's item called free function

    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            //younger is not empty, old is empty
            use std::mem::swap;
            //bring the elements in younger to older
            swap(&mut self.older, &mut self.younger);
            //make it FIFO
            self.older.reverse();
        }
        self.older.pop()
    }

    fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

impl<T> Queue<T> {
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

#[test]
fn test_queue() {
    let mut q: Queue<char> = Queue { older: Vec::new(), younger: Vec::new() };
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('$');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('$'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());
}

use std::rc::Rc;

struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }

    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self)
    }
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Self { older: Vec::new(), younger: Vec::new() }
    }
}

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 1.0 };
}

//given any specific lifetime 'elt, you can make Extrema<'elt>
// that holds references with that lifetime
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}


//it’s so common for the return type to use the same lifetime as an argument
//Rust lets us omit the lifetimes when there’s one obvious candidate.

//fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for item in slice.iter().skip(1) {
        if item < least {
            least = &item;
        }
        if item > greatest {
            greatest = &item;
        }
    }
    Extrema { greatest, least }
}

#[test]
fn test_find_extrema() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years,
}

impl TimeUnit {
    fn plural(self) ->&'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours   => "hours",
            TimeUnit::Days    => "days",
            TimeUnit::Months  => "months",
            TimeUnit::Years   => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}
