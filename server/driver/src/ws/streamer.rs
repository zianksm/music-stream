
use actix::{
    Actor, ActorContext, ActorFutureExt, AsyncContext, Handler, Message, StreamHandler,
};
use actix_web_actors::ws;
use serde_json::Value;

use super::protocol::ContextMapper;
pub struct Streamer;

#[derive(Message)]
#[rtype(result = "()")]
struct SimpleMessage(pub String);


impl Handler<SimpleMessage> for Streamer {
    type Result = ();

    fn handle(&mut self, msg: SimpleMessage, ctx: &mut Self::Context) -> Self::Result {
        let value = format!("msg received: {}", msg.0);
        ctx.text(value)
    }
}

impl Actor for Streamer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("actor started");

        let msg = String::from("connection established");
        let msg = SimpleMessage(msg);

        ctx.address().do_send(msg)
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        println!("stopping actor");

        let msg = String::from("disconnected");
        let msg = SimpleMessage(msg);

        ctx.address().do_send(msg);

        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Streamer {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        print!("msg received: {:?}", item);

        match item {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Text(msg)) => Self::handle_msg(msg.to_string(), ctx),
            Ok(ws::Message::Close(_reason)) => ctx.stop(),
            _ => (),
        }
    }
}

impl Streamer {
    fn handle_msg(msg: String, ctx: &mut ws::WebsocketContext<Streamer>) {
        let Ok(value) = serde_json::from_str::<Value>(&msg) else{
            ctx.address().do_send(SimpleMessage(msg));
            return;
        };

        let _msg = ContextMapper::map(&value).unwrap();
    }
}
