use crate::config::config_redis::get_config_redis;
use cached::proc_macro::cached;

#[cached]
pub fn get_depends_redis_client() -> redis::Client {
    let config_redis = get_config_redis();
    redis::Client::open(
        format!(
            "redis://{}:{}",
            config_redis.redis_url, config_redis.redis_port
        )
        .as_str(),
    )
    .unwrap()
}
