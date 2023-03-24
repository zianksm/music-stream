use actix::{Actor, ActorContext, ActorFutureExt, AsyncContext, Handler, Message, StreamHandler};
use actix_web_actors::ws::{self, CloseReason};
use serde_json::Value;

use super::{message_handler::ProtocolMessageHandler, protocols::contexts::mapper::ContextMapper};

pub struct Streamer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SimpleMessage(pub String);

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
        actix::Running::Stop
    }


}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Streamer {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Text(msg)) => Self::handle_msg(msg.to_string(), ctx),
            Ok(ws::Message::Close(reason)) => Self::handle_stop(ctx, reason),
            _ => (),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        // do nothing so that the connection can keep being alive
        ()
    }
}

impl Streamer {
    fn handle_msg(msg: String, ctx: &mut ws::WebsocketContext<Streamer>) {
        let Ok(value) = serde_json::from_str::<Value>(&msg) else{
            ctx.address().do_send(SimpleMessage(msg));
            return;
        };

        //TODO : implement streams
        let result = ContextMapper::map(&value).unwrap().execute().unwrap();

        ProtocolMessageHandler::handle(result, ctx);
    }

    fn handle_stop(ctx: &mut ws::WebsocketContext<Streamer>, reason: Option<CloseReason>) {
        println!(
            "stopping actor because : {:?}",
            reason
                .unwrap()
                .code
        );

        let msg = String::from("disconnected");
        let msg = SimpleMessage(msg);

        ctx.address().do_send(msg);

        ctx.stop()
    }
}
