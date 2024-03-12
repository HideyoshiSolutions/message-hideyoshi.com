use std::sync::Arc;
use crate::config::config_email::ConfigEmail;
use crate::model::send_message::SendMessage;
use lettre::message::Mailbox;
use lettre::{
    transport::smtp::authentication::Credentials, Address, AsyncSmtpTransport, AsyncTransport,
    Message, Tokio1Executor,
};


type MessageQueue = deadqueue::unlimited::Queue<SendMessage>;


#[derive(Clone)]
pub struct EmailService {
    name: String,
    email: String,
    mailer: AsyncSmtpTransport<Tokio1Executor>,

    message_queue: Arc<MessageQueue>
}

impl EmailService {
    pub fn new(config_email: ConfigEmail) -> Self {
        let email_service = EmailService {
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
            message_queue: Arc::new(MessageQueue::new())
        };
        email_service.bind_executer();

        email_service
    }

    pub async fn send_email (
        &self,
        m: SendMessage
    ) {
        self.message_queue.push(m);
    }

    fn bind_executer(&self) {
        let local_self = self.clone();
        let message_queue = self.message_queue.clone();
        tokio::spawn(async move {
            loop {
                let message = message_queue.pop().await;
                local_self.create_send_message_task(message).await;
            }
        });
    }

    async fn create_send_message_task(&self, m: SendMessage) {
        let message = self.message_queue.pop().await;
        match self.send_message_smtp(message.clone()).await {
            Ok(_) => {
                println!(
                    "Email sent successfully from {} to {}",
                    message.author.unwrap().email,
                    self.email
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
}
