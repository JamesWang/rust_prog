macro_rules! comp {
    ($f: expr) => {
        move |g: fn(_) -> _| move |x: _| $f(g(x))
    };
}

pub fn tap<'a, T>(f: &'a dyn Fn(&mut T) -> &mut T, value: &'a mut T) -> &'a mut T {
    f(value);
    value
}

#[test]
fn test_add7_multi3_eqs_39() {
    let add7 = |x| x + 7;
    let mul3 = |x| x * 3;
    let add7mul3 = comp!(mul3)(add7);
    println!("{}", add7mul3(6));
    assert_eq!(39, add7mul3(6))
}