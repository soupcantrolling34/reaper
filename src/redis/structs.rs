pub struct RedisError {
    pub message: String,
    pub redis_error: Option<redis::RedisError>
}

impl std::fmt::Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}