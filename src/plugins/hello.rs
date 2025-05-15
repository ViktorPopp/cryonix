use actix_web::{HttpResponse, Responder};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>Hello from the plugin!</h1>")
}
