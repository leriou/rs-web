use super::middware::redis_di;

pub struct RedisSrv {
    conn: redis::RedisResult<redis::Connection>
};

impl RedisSrv {
    pub fn New() -> Self {
        RedisSrv {
        }
    }
}
