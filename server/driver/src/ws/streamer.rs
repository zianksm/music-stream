use actix::ContextFutureSpawner;
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, AsyncContext, Handler, Message, StreamHandler,
    WrapFuture,
};
use actix_web_actors::ws;
pub struct Streamer;

#[derive(Message)]
#[rtype(result = "()")]
struct SimpleMessage(pub String);

impl Handler<SimpleMessage> for Streamer {
    type Result = ();

    fn handle(&mut self, msg: SimpleMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0)
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
            Ok(ws::Message::Text(msg)) => ctx.text(msg),
            Ok(ws::Message::Close(reason)) => ctx.stop(),
            _ => (),
        }
    }
}
