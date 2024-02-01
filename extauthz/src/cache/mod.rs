use core::fmt;
use deadpool_redis::{Config, Runtime};

use tokio::sync::OnceCell;

pub mod rate_limit;

static CACHE: OnceCell<Cache> = OnceCell::const_new();

pub struct Cache(deadpool_redis::Pool);

impl Cache {
    pub async fn init(host: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config {
            url: Some(format!("redis://default:{password}@{host}/")),
            connection: None,
            pool: Some(deadpool_redis::PoolConfig::new(50)),
        };

        let cache = Cache(config.create_pool(Some(Runtime::Tokio1))?);
        CACHE.set(cache)?;
        Ok(())
    }

    async fn get_connection() -> Result<deadpool_redis::Connection, Box<dyn std::error::Error>> {
        let cache = CACHE.get().expect("Cache must be initialized");
        let Cache(pool) = cache;
        Ok(pool.get().await?)
    }
}

impl fmt::Debug for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Cache(pool)")
    }
}
