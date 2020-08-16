pub struct RedisClient;

impl RedisClient {
    pub fn New() -> redis::RedisResult<redis::Connection> {
        RedisClient::NewWithParams("127.0.0.1", 6379usize)
    }

    pub fn NewWithParams(host: &str, port: usize) -> redis::RedisResult<redis::Connection> {
        let cli = redis::Client::open(format!("redis://{}:{}/", host, port))?;
        cli.get_connection()
    }
}
