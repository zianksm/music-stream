use actix_web::{
    web::{self, route},
    App, HttpServer,
};

mod api;
pub mod ws;

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(api::hello::hello))
            .route("/stream", web::get().to(api::music::stream))
            .route("/ws/stream", web::get().to(ws::endpoint::connect))
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}
