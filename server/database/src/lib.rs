use config::ServerConfig;
use sqlx::{mysql, MySql, Pool};

pub struct Database {
    config: ServerConfig,
    pool: Pool<MySql>,
}

impl Database {
    pub async fn init() -> Result<Self, sqlx::Error> {
        let config = ServerConfig::new();

        let pool = mysql::MySqlPoolOptions::new()
            .max_connections(10)
            .connect(config.db_url())
            .await?;

        let _self = Self { config, pool };

        Ok(_self)
    }

    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    pub fn pool(&self) -> &Pool<MySql> {
        &self.pool
    }
}
