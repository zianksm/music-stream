use serde::Deserialize;
use serde_json::Value;

use crate::ws::protocols::erorr::ErrorAdapter;

#[derive(Deserialize)]
pub struct Spec {
    spec: String,
}

impl Spec {
    pub fn new(val: &Value) -> Result<Self, anyhow::Error> {
        let Some(spec) = val.get("spec") else { 
            let err = ErrorAdapter::make("invalid protocol spec, must specify a spec field");
            return Err(err);
        };

        let spec = spec.to_string();

        Ok(Self { spec })
    }

    pub fn spec(&self) -> &str {
        self.spec.as_ref()
    }
}
