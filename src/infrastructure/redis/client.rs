use redis::{
    aio::MultiplexedConnection, Client, RedisResult
};


pub async fn init_redis_connection() -> RedisResult<MultiplexedConnection> {
    let client = Client::open("redis://127.0.0.1/")?;
    let conn = client.get_multiplexed_tokio_connection().await?;
    Ok(conn)
}