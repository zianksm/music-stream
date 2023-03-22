use actix_web::{web, HttpRequest, Responder};
use actix_web_actors::ws;

use super::streamer::Streamer;

pub async fn connect(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let res = ws::start(Streamer {}, &req, stream);

    println!("{:?}", res);
    res
}
