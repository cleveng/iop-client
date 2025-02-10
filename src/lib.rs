use constants::caches;
use deadpool_redis::{redis::cmd, Runtime};
use log::info;
use reqwest::Client;

mod constants;
mod core;
mod model;
mod photobank;
mod product_category;
mod product_country;
mod product_group;
mod token;

#[derive(Clone)]
pub struct IopClient {
    appid: String,
    app_secret: String,
    pool: deadpool_redis::Pool,
    client: Client,
}

impl IopClient {
    /// Creates a new instance of `IopClient`.
    ///
    /// This function initializes an `IopClient` with the specified application ID,
    /// application secret, and Redis address. It attempts to create a connection pool
    /// to the Redis server using the provided address and verifies the connection by
    /// setting a test key. If the connection is successful, an `IopClient` object is
    /// returned; otherwise, the function panics with an error message.
    ///
    /// # Arguments
    ///
    /// * `appid` - The application ID for the client.
    /// * `app_secret` - The secret key associated with the application ID.
    /// * `redis_addr` - The address of the Redis server to connect to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `IopClient` instance if successful, or an error if the
    /// Redis connection could not be established.
    pub async fn new(
        appid: String,
        app_secret: String,
        redis_addr: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let cfg = deadpool_redis::Config::from_url(redis_addr);
        let pool = match cfg.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => pool,
            Err(err) => {
                panic!("Failed to create redis pool: {err}")
            }
        };

        match pool.get().await {
            Ok(mut conn) => {
                match cmd("SETEX")
                    .arg("PING")
                    .arg(caches::FIVE_MINUTE_IN_SECONDS)
                    .arg("pong")
                    .query_async::<()>(&mut conn)
                    .await
                {
                    Ok(_) => {
                        info!("Redis connected");
                    }
                    Err(err) => {
                        panic!("Failed to connect to redis: {err}")
                    }
                }
            }
            Err(err) => {
                panic!("Failed to connect to redis: {err}")
            }
        };

        Ok(IopClient {
            appid,
            app_secret,
            pool,
            client: Client::new(),
        })
    }
}
