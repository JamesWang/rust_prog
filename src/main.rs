use num::Complex;

mod mandelbrot;
mod rhttp;

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

fn main() {
    let my_list = ["One", "Two", "Three"];
    for n in &my_list {
        println!("{}", n);
    }
}
