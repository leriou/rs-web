use crate::middware::redis_di;

pub struct RedisSrv {
    conn: redis::Connection,
}

impl RedisSrv {
    pub fn New() -> Self {
        RedisSrv {
            conn: redis_di::RedisClient::New().unwrap(),
        }
    }
}
