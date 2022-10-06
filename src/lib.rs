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
