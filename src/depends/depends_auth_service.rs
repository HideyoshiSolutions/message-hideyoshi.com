use cached::proc_macro::cached;

use crate::config::config_auth::get_config_auth;
use crate::config::config_limits::get_config_limits;
use crate::depends::depends_redis_client::get_depends_redis_client;
use crate::service::auth_service::AuthService;

#[cached]
pub fn get_depends_auth_service() -> AuthService {
    AuthService::new(
        get_config_auth(),
        get_depends_redis_client(),
        get_config_limits(),
    )
}
