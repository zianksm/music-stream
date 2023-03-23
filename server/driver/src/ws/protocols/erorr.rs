use anyhow::anyhow;

pub struct ErrorAdapter;

impl ErrorAdapter {
    pub fn make<T: ToString>(err: T) -> anyhow::Error {
        anyhow!("{}", err.to_string())
    }
}
