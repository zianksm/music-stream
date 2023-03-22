pub struct ServerConfig {
    db_local: String,
    db_test: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let db_local = std::env::var("DB_LOCAL").expect("missing db local url");
        let db_test = std::env::var("DB_TEST").expect("missing db test url");

        {
            Self { db_local, db_test }
        }
    }

    pub fn db_url(&self) -> &str {
        self.db_local.as_ref()
    }

    pub fn db_test(&self) -> &str {
        self.db_test.as_ref()
    }
}

