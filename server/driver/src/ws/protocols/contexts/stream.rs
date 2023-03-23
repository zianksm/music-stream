use serde::Deserialize;
use serde_json::Value;

use crate::ws::protocols::{
    enums::ProtocolMessage,
    erorr::ErrorAdapter,
    traits::{ActionContext, CreationContext},
};

#[derive(Deserialize)]
pub struct StreamContext {
    name: String,
}

impl StreamContext {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl ActionContext for StreamContext {
    fn exec() -> Result<ProtocolMessage, anyhow::Error> {
        todo!()
    }
}

impl CreationContext<Self> for StreamContext {
    fn new(val: &Value) -> Result<Self, anyhow::Error> {
        let Some(name) = val.get("name") else { 
            let err = ErrorAdapter::make("invalid context, must specify a name field in a stream spec");
            return Err(err);
        };

        let name = name.to_string();

        Ok(Self { name })
    }
}
