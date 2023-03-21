// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

pub struct ServerConfig {
    db_url: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let db_url = std::env::var("DB_URL").expect("missing db url");

        {
            Self { db_url }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_load_env() {
        let result = ServerConfig::new();
        assert!(!result.db_url.is_empty())
    }
}
