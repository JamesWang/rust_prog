fn call_twice<F>(mut closure: F) where F : FnMut() {
    closure();
    closure();
}

pub fn call_main() {
    let my_str = "hello".to_string();
    let mut i = 0;
    //let f = |mut my: String| drop(my);
    //call_twice(f)
    call_twice(|| i+=1);

    print!("i={}\n", i);
}