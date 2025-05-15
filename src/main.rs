use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod plugins;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/", web::get().to(plugins::hello::hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
