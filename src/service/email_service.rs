use lettre::message::Mailbox;
use lettre::{
    transport::smtp::authentication::Credentials, Address, AsyncSmtpTransport, AsyncTransport,
    Message, Tokio1Executor,
};
use redis_queue_rs::async_redis_queue::AsyncRedisQueue;

use crate::config::config_email::ConfigEmail;
use crate::model::send_message::SendMessage;

#[derive(Clone)]
pub struct EmailService {
    name: String,
    email: String,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    redis_client: redis::Client,
}

impl EmailService {
    pub fn new(config_email: ConfigEmail, redis_client: redis::Client) -> EmailService {
        let mailer =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config_email.smtp_server)
                .unwrap()
                .credentials(Credentials::new(
                    config_email.smtp_username,
                    config_email.smtp_password,
                ))
                .port(config_email.smtp_port)
                .build();

        EmailService {
            mailer,
            redis_client,
            name: config_email.smtp_name.clone(),
            email: config_email.smtp_email.clone(),
        }
    }

    pub async fn send_email(&mut self, m: SendMessage) {
        self.get_message_queue().await.push(m).await;
    }

    pub(crate) async fn create_send_message_task(&mut self) {
        let message = self.get_message_queue().await.pop().await;
        if message.is_none() {
            return;
        }

        let message = message.unwrap();
        let message_author = message.author.clone().unwrap();
        match self.send_message_smtp(message).await {
            Ok(_) => {
                println!(
                    "Email sent successfully from {} to {}",
                    message_author.email, self.email
                );
            }
            Err(e) => {
                println!("Error sending email: {}", e);
            }
        }
    }

    async fn send_message_smtp(&self, m: SendMessage) -> Result<(), Box<dyn std::error::Error>> {
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

    pub async fn get_message_queue(&self) -> AsyncRedisQueue<SendMessage> {
        let redis_client = self.redis_client.clone();
        AsyncRedisQueue::new("message-hideyoshi.com".to_string(), redis_client).await
    }
}
