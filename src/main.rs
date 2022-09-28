mod rhttp;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use rhttp::rhttp;
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
