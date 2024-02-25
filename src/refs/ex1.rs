use std::string::ToString;

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

const ARIA_STR: &str = "Aria: The Animation";

#[test]
fn ex1_f1() {
    let aria = Anime { name: ARIA_STR, bechdel_pass: true };
    let anime_ref = &aria;

    assert_eq!(anime_ref.name, ARIA_STR);
    assert_eq!((*anime_ref).name, ARIA_STR)
}

#[test]
fn ex1_f2(){
    let mut v = vec![1973, 1968];
    v.sort();
    (&mut v).sort(); //equivalent to above, more verbose

    let x = 10;
    let y = 20;
    let b = true;
    let mut r = &x;

    if b {r = &y;}
    assert!(*r == 10 || *r == 20);
}

#[test]
fn ex1_f3_ref_to_ref(){
    struct Point {x: i32, y: i32}
    let point = Point{x: 1000, y: 729};
    let r: &Point = &point;
    let rr: &&Point  = &r;
    let rrr: &&&Point = &rr;
    //rrr -> rr -> r
    assert_eq!(rr.y, r.y);
    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    //comparison must have exactly the same type
    //Rust references are never null, and no default initial value for a reference
    //cannot convert integers to references, cannot convert 0 into a reference
}

#[test]
fn ex1_f3() {
    fn factorial(n: usize) -> usize {
        (1..n+1).product()
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}
static mut STASH: &i32 = &128;

#[test]
fn f_test() {
    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }
    f(&15)
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r;}
    }
    s
}

#[test]
fn test_smallest(){
    let s;
    {
        let parabola = [9,4,1,0,1,4,9];
        s = smallest(&parabola);
        assert_eq!(*s, 0)
    }
}

struct S<'a> {
    r: &'a i32
}

#[test]
fn test_s(){
    let x = 10;
    let s = S{r: &x};
    assert_eq!(*s.r, 10);
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len(){
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i])
            }
        }
        None
    }
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

#[test]
fn test_extend(){
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = vec![0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
}

#[test]
fn reborrow_ok() {
    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;
    let r1 = &m.1;
    //v.1; borrowed by m
    println!("{}", r1);
}

