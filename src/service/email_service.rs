use crate::config::config_email::ConfigEmail;
use crate::model::send_message::SendMessage;
use lettre::message::Mailbox;
use lettre::{
    transport::smtp::authentication::Credentials, Address, AsyncSmtpTransport, AsyncTransport,
    Message, Tokio1Executor,
};

#[derive(Clone)]
pub struct EmailService {
    name: String,
    email: String,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailService {
    pub fn new(config_email: ConfigEmail) -> Self {
        EmailService {
            name: config_email.smtp_name.clone(),
            email: config_email.smtp_email.clone(),
            mailer: AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config_email.smtp_server)
                .unwrap()
                .credentials(Credentials::new(
                    config_email.smtp_username,
                    config_email.smtp_password,
                ))
                .port(config_email.smtp_port)
                .build(),
        }
    }

    pub async fn send_email_smtp(&self, m: SendMessage) -> Result<(), Box<dyn std::error::Error>> {
        let sender = m.author.clone().unwrap();
        let sender_mailbox =
            Mailbox::new(Some(sender.name), sender.email.parse::<Address>().unwrap());

        let recipient_mailbox = Mailbox::new(
            Some(self.name.clone()),
            self.email.parse::<Address>().unwrap(),
        );

        let body = self.create_email_body(&m);

        let email = Message::builder()
            .from(sender_mailbox.clone())
            .to(recipient_mailbox.clone())
            .subject(m.subject)
            .body(body.to_string())?;

        self.mailer.send(email).await?;

        Ok(())
    }

    fn create_email_body(&self, m: &SendMessage) -> String {
        let sender = m.author.clone().unwrap();

        format!(
            "From: {} <{}>\nTo: {} <{}>\nSubject: {}\n\n{}",
            sender.name, sender.email, self.name, self.email, m.subject, m.message
        )
    }
}
