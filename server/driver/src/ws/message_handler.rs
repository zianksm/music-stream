use std::time::Duration;

use actix::{clock, AsyncContext};
use actix_web::Either;
use actix_web_actors::ws::{self, ProtocolError, WebsocketContext};
use bytes::Bytes;
use futures::Stream;
use futures::{task::Poll, FutureExt};

use super::{
    protocols::enums::ProtocolMessage,
    streamer::{SimpleMessage, Streamer},
};

pub struct ProtocolMessageHandler;

pub struct StreamWrapper(pub Vec<u8>);

impl Iterator for StreamWrapper {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let size = 500 * 1024; // send 512 kb chunks of data

        println!("{}", self.0.len());

        match self.0.len() {
            0 => None,
            _ => Some(self.0.drain(0..size.min(self.0.len())).collect::<Vec<u8>>()),
        }
    }
}

impl Stream for StreamWrapper {
    type Item = Result<actix_web_actors::ws::Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let inner = self.get_mut();

        match inner.next() {
            Some(item) => Poll::Ready(Some(Ok(ws::Message::Binary(Bytes::from(item))))),
            None => Poll::Ready(None),
        }
    }
}

impl ProtocolMessageHandler {
    pub fn handle(msg: ProtocolMessage, ctx: &mut WebsocketContext<Streamer>) {
        match msg {
            ProtocolMessage::Bytes(data) => Self::handle_binary(data, ctx),
            ProtocolMessage::Text(data) => ctx.address().do_send(SimpleMessage(data)),
        }
    }

    fn handle_binary(data: bytes::Bytes, ctx: &mut WebsocketContext<Streamer>) {
        let bytes = data.to_vec();
        let stream = StreamWrapper(bytes);

        ctx.add_stream(stream);
    }
}
