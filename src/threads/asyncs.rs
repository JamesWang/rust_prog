use std::{thread, time};
use actix_web::web::block;
use futures::executor::block_on;

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}
pub async fn async_run() {
    let now = time::Instant::now();
    //let f_one: i8 = do_something(1);
    //let outcome = block_on(f_one);
    let f_two = async {
        let two: i8 = do_something(2).await;
        let three: i8 = do_something(3).await;
        return [two, three]
    };
    let r2 = block_on(f_two);
    println!("time elapsed {:?}", now.elapsed());
    println!("outcome {:?}", r2);
}
use std::vec::Vec;
use async_std;
use futures::future::join_all;

pub async fn async_run2() {
    let third_outcome = async  {
      let mut futures_vec = Vec::new();
        let future_4 = do_something(4);
        let future_5 = do_something(5);
        futures_vec.push(future_4);
        futures_vec.push(future_5);
        let handles = futures_vec.into_iter().map(
            async_std::task::spawn
        ).collect::<Vec<_>>();
        return join_all(handles).await;
    };
    let now = time::Instant::now();
    let result = block_on(third_outcome);
    println!("time elapsed {:?}", now.elapsed());
    println!("result {:?}", result);
}