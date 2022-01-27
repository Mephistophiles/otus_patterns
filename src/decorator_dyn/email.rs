use lettre::{Message, SmtpTransport, Transport};

use super::Notifier;

pub struct EmailNotifier {
    email: String,
    subject: String,
    notifier: Option<Box<dyn Notifier>>,
}

impl EmailNotifier {
    // TODO: clippy warns on new(): https://rust-lang.github.io/rust-clippy/master/index.html#new_ret_no_self
    pub fn new_notifier(
        to: impl Into<String>,
        subject: impl Into<String>,
        notifier: Option<Box<dyn Notifier>>,
    ) -> Box<dyn Notifier> {
        Box::new(Self {
            email: to.into(),
            subject: subject.into(),
            notifier,
        })
    }
}

impl Notifier for EmailNotifier {
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
