use actix_web::{web, App, HttpServer};

mod api;

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(api::hello::hello))
            .route("/stream", web::get().to(api::music::stream))
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}
