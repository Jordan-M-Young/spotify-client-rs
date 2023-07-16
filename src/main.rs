use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
pub mod config;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::load_toml("./config.toml");

    let port = config.app.port;



    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",port))?
    .run()
    .await
}
