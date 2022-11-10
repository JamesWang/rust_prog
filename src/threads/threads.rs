use std::{thread, time};
use std::thread::JoinHandle;

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

pub fn run_threads() {
    let now = time::Instant::now();
    let one: i8 = do_something(1);
    let two: i8 = do_something(2);
    let three: i8 = do_something(3);

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", one + two + three);
}

pub fn run_threads2() {
    let now = time::Instant::now();
    let th_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
    let th_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
    let th_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

    let r1 = th_one.join();
    let r2 = th_two.join();
    let r3 = th_three.join();

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", r1.unwrap() + r2.unwrap() + r3.unwrap());
}
