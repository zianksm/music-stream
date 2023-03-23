use serde_json::Value;

use crate::ws::protocols::enums::Protocol;

use super::protocol_context::ProtocolContext;

pub struct ContextMapper;

impl ContextMapper {
    pub fn map(value: &Value) -> Result<Protocol, anyhow::Error> {
        let ctx = ProtocolContext::new(value)?;
        let protocol = Protocol::infer(value, &ctx)?;

        Ok(protocol)
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_should_map_protocols() {
        let json = json!(
            {

             "spec":"stream",
             "name": "yume"
            }

        );

        let result = ContextMapper::map(&json).unwrap();
        assert!(result.is_stream());
    }
}
