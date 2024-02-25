use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use crate::config::config_email::ConfigEmail;


#[derive(Clone)]
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>
}

impl EmailService {
    pub fn new(config_email: ConfigEmail) -> Self {
        EmailService {
            mailer: AsyncSmtpTransport::<Tokio1Executor>::relay(&config_email.smtp_server)
                .unwrap()
                .credentials(Credentials::new(config_email.smtp_username, config_email.smtp_password))
                .port(config_email.smtp_port)
                .build()
        }
    }

    async fn send_email_smtp(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_string())?;

        self.mailer.send(email).await?;

        Ok(())
    }
}
