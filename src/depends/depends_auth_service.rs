use crate::config::config_auth::get_config_auth;
use crate::config::config_limits::get_config_limits;
use crate::config::config_redis::get_config_redis;
use crate::service::auth_service::AuthService;
use cached::proc_macro::cached;

#[cached]
pub fn get_depends_auth_service() -> AuthService {
    AuthService::new(get_config_auth(), get_config_redis(), get_config_limits())
}
