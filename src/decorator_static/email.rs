use lettre::{Message, SmtpTransport, Transport};

use super::Notifier;

pub struct EmailNotifier<T: Notifier> {
    email: String,
    subject: String,
    notifier: Option<T>,
}

impl<T: Notifier> EmailNotifier<T> {
    pub fn new(to: impl Into<String>, subject: impl Into<String>, notifier: Option<T>) -> Self {
        Self {
            email: to.into(),
            subject: subject.into(),
            notifier,
        }
    }
}

impl<T: Notifier> Notifier for EmailNotifier<T> {
    fn send(&self, message: &str) {
        let email = Message::builder()
            .from("Notifier <notifier@nobody.com>".parse().unwrap())
            .to(self.email.parse().unwrap())
            .subject(&self.subject)
            .body(message.to_string())
            .unwrap();

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.global.org").unwrap().build();

        // Send the email
        mailer.send(&email).ok();

        if let Some(notifier) = &self.notifier {
            notifier.send(message);
        }
    }
}
