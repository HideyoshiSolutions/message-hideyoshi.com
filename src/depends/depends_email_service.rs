use cached::proc_macro::cached;
use crate::config::config_email::get_config_email;
use crate::service::email_service::EmailService;


#[cached]
pub fn get_depends_email_service() -> EmailService {
    EmailService::new(
        get_config_email()
    )
}
