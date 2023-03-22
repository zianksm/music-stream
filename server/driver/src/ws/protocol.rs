use std::str::FromStr;

use anyhow::anyhow;
use serde::Deserialize;
use serde_json::Value;

// TODO : make a macro that recursively search and match enum members to check it if exist
// and maybe add a proc(attribute/derive) to automatically create an instance(with a trait) and propagate erorrs(using anyhow)
pub enum Protocol {
    STREAM(StreamContext),
}

impl Protocol {
    pub fn infer(val: &Value, ctx: &ProtocolContext) -> Result<Protocol, anyhow::Error> {
        let spec = ctx.spec.to_lowercase();
        let cmp = "stream";

        println!("{}", cmp.eq(spec.as_str()));

        match spec.as_str() {
            cmp => Self::handle_stream_creation(val),
            _ => Err(ErrorAdapter::make(format!(
                "invalid spec command, got : {}",
                spec
            ))),
        }
    }

    fn handle_stream_creation(val: &Value) -> Result<Protocol, anyhow::Error> {
        let ctx = StreamContext::new(val)?;
        Ok(Self::STREAM(ctx))
    }

    /// Returns `true` if the protocol is [`STREAM`].
    ///
    /// [`STREAM`]: Protocol::STREAM
    #[must_use]
    pub fn is_stream(&self) -> bool {
        matches!(self, Self::STREAM(..))
    }
}

#[derive(Deserialize)]
pub struct ProtocolContext {
    spec: String,
}

impl ProtocolContext {
    pub fn new(val: &Value) -> Result<Self, anyhow::Error> {
        let Some(spec) = val.get("spec") else { 
            let err = ErrorAdapter::make("invalid protocol spec, must specify a spec field");
            return Err(err);
        };

        let spec = spec.to_string();

        return Ok(Self { spec });
    }
}

#[derive(Deserialize)]
pub struct StreamContext {
    name: String,
}

impl StreamContext {
    pub fn new(val: &Value) -> Result<Self, anyhow::Error> {
        let Some(name) = val.get("name") else { 
            let err = ErrorAdapter::make("invalid context, must specify a name field in a stream spec");
            return Err(err);
        };

        let name = name.to_string();

        return Ok(Self { name });
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

pub struct ContextMapper;

impl ContextMapper {
    pub fn map(value: &Value) -> Result<Protocol, anyhow::Error> {
        let ctx = ProtocolContext::new(&value)?;
        let protocol = Protocol::infer(&value, &ctx)?;

        Ok(protocol)
    }
}

pub struct ErrorAdapter;

impl ErrorAdapter {
    fn make<T: ToString>(err: T) -> anyhow::Error {
        anyhow!("{}", err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_map_protocols() {
        let json = "
        {
         \"spec\":\"stream\",
         \"name\": \"yume no tsuzuki\"   
        }
        ";

        let json = serde_json::from_str::<Value>(&json).unwrap();
        let result = ContextMapper::map(&json).unwrap();
        assert!(result.is_stream());
    }
}
