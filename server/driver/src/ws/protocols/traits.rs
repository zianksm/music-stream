use serde_json::Value;

use super::{contexts::protocol_context::Spec, enums::ProtocolMessage};

pub trait ActionContext {
    fn exec() -> Result<ProtocolMessage, anyhow::Error>;
}

pub trait CreationContext<T: ActionContext> {
    fn new(val: &Value) -> Result<T, anyhow::Error>;
}

pub trait ResolveContext {
    fn is(ctx: &Spec) -> bool;
}
