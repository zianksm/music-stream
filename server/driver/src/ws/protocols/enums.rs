use serde_json::Value;

use super::{
    contexts::{protocol_context::Spec, stream::StreamContext},
    erorr::ErrorAdapter,
    traits::{ActionContext, CreationContext, ResolveContext},
};

pub struct Protocol(Box<dyn ActionContext>);

pub enum ProtocolMessage {
    Bytes(bytes::Bytes),
    Text(String),
}

impl ProtocolMessage {
    /// Returns `true` if the protocol message is [`Bytes`].
    ///
    /// [`Bytes`]: ProtocolMessage::Bytes
    #[must_use]
    pub fn is_bytes(&self) -> bool {
        matches!(self, Self::Bytes(..))
    }

    /// Returns `true` if the protocol message is [`Text`].
    ///
    /// [`Text`]: ProtocolMessage::Text
    #[must_use]
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(..))
    }
}

impl Protocol {
    pub fn infer(val: &Value, ctx: &Spec) -> Result<Protocol, anyhow::Error> {
        match ctx {
            _x if StreamContext::is(ctx) => Ok(Self(Box::new(StreamContext::new(val)?))),
            _ => Err(Self::err(ctx)),
        }
    }

    fn err(ctx: &Spec) -> anyhow::Error {
        let str = format!("invalid spec command, got : {}", ctx.spec());
        

        ErrorAdapter::make(str)
    }
}

impl Protocol {
    pub fn execute(&self) -> Result<ProtocolMessage, anyhow::Error> {
        self.0.exec()
    }
}
