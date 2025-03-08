use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

async fn man_h() -> impl Responder {
    HttpResponse::Ok().body("impl hi!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .route("/impl", web::get().to(man_h))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}