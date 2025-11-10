use redis::{
    aio::MultiplexedConnection, Client, RedisResult
};

use crate::ENV;


pub async fn init_redis_connection() -> RedisResult<MultiplexedConnection> {
    let client = Client::open(ENV.redis_uri.clone())?;
    let conn = client.get_multiplexed_tokio_connection().await?;
    Ok(conn)
}