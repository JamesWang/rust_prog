#![warn(clippy::all, clippy::pedantic)]

use num::Complex;
use crate::algo::algos::max_diff;
use crate::examples::example::{Point, tweet_test};
use crate::refs::show::sort_works;
use crate::to_do::structs::traits::create::Create;

mod mandelbrot;
mod rhttp;
mod refs;
mod algo;
mod args;
mod macros;
mod examples;
mod to_do;
mod actix_hw;
mod state;
mod threads;
mod grep;
mod json;
mod heap_graph;


#[actix_web::main]
async fn ax_web() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use rhttp::rhttp_impl as rhttp;

    HttpServer::new(|| {
        App::new()
            .service(rhttp::get_index)
            .service(rhttp::post_gcd)
    })
        .bind("127.0.0.1:3000")
        .expect("Bind ")
        .run()
        .await
}

fn mandelbrot_image() {
    use mandelbrot::images as mimage;
    use mandelbrot::mandelbrot_impl as md;
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }
    let bounds = md::parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = md::parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = md::parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    md::quicker_render(&mut pixels, bounds, upper_left, lower_right);
    mimage::write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn main1() {
    mandelbrot_image();
}

fn main2() {
    let my_list = ["One", "Two", "Three"];
    for n in &my_list {
        println!("{}", n);
    }
}

fn main3() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let third = &v[2]; // cannot use v[2]; because otherwise Rust has to remember v[2] is uninitialized after this line
    let fifth = &v[4]; // same here
}

fn check_to_do() {
    use to_do::ItemTypes;
    use to_do::to_do_factory;
    use to_do::structs::traits::create::Create;

    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");
    /*match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title, "".to_string, ),
        ItemTypes::Done(item)      => println!("it's a done item with the title: {}", item.super_struct.title)
    }*/
}
//#[actix_web::main]
fn main() {
    //use args::args_ex::args_main;
    //show_table();
    //args_main();

    //check_to_do();
    //use actix_hw::basic_hw;
    //basic_hw::echo_server().await
    //basic_hw::echo_server().await.expect("TODO: panic message");
    //basic_hw::start_it().await
    //use state::run_it;
    //run_it()
    //threads::threads::run_threads2();
    //threads::asyncs::async_run2().await;
    /*{
        use examples::example::Point;
        let p = Point {x: 5, y: 10};
        println!("p.x={}", p.x());
        let p2 = Point { x: 3.0, y: 4.0};
        println!("p.distance={}", p2.distance_from_origin())
    }*/
    //tweet_test();

    heap_graph::hgraph::graph_main();
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn show_table() {
    use refs::show::{showing, Table, sort_works, factorial};

    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()],
    );
    table.insert(
        "Cellini".to_string(),
        vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()],
    );
    sort_works(&mut table);
    showing(&table);
}
