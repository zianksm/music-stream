use std::io::Read;


use serde::Deserialize;
use serde_json::Value;

use crate::ws::protocols::{
    enums::ProtocolMessage,
    erorr::ErrorAdapter,
    traits::{ActionContext, CreationContext, ResolveContext},
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
    fn exec(&self) -> Result<ProtocolMessage, anyhow::Error> {
        let path = std::env::current_dir()
            .unwrap()
            .join("music")
            .join("yume no tsuzuki.mp3");

        let file = std::fs::File::open(path).unwrap();
        let len = file.metadata().unwrap().len() as usize;

        let mut buffer: Vec<_> = Vec::with_capacity(len);
        let mut reader = std::io::BufReader::new(file);

        reader.read_to_end(&mut buffer);

        let data = bytes::Bytes::from(buffer);
        
        Ok(ProtocolMessage::Bytes(data))
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

impl ResolveContext for StreamContext {
    fn is(ctx: &super::protocol_context::Spec) -> bool {
        match ctx.spec().to_lowercase().as_str() {
            "stream" | "\"stream\"" => true,
            _ => false,
        }
    }
}
