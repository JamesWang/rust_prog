use actix_web::{App, HttpResponse, HttpServer, Responder, web, get, post};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world rust")
}

pub async fn start_actix_server() -> std::io::Result<()> {
    println!("Starting actix server....");
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8087))?
        .run()
        .await
}

#[post("/echo")]
async fn echo(request: String) -> impl Responder {
    HttpResponse::Ok().body(request)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn echo_server() -> std::io::Result<()> {
    HttpServer::new(||
        App::new().service(echo).route("/hey", web::get().to(manual_hello)))
        .bind("0.0.0.0:8087")?.run().await
}