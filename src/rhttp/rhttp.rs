use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use serde::Deserialize;
use rust_prog::gcd;

#[derive(Deserialize)]
pub struct GcdParameters {
    pub n: u64,
    pub m: u64,
}


pub fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}

pub fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    let response =
        format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok().content_type("text/html").body(response)
}

pub fn create_server() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
        .bind("127.0.0.1:3000")
        .expect("Error binding to address")
        .run()
        .expect("Error to create server")
}
