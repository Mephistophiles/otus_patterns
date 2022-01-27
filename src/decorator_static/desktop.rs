use notify_rust::Notification;

use super::Notifier;

pub struct DesktopNotifier<T: Notifier> {
    subject: String,
    notifier: Option<T>,
}

impl<T: Notifier> DesktopNotifier<T> {
    pub fn new(subject: impl Into<String>, notifier: Option<T>) -> Self {
        Self {
            subject: subject.into(),
            notifier,
        }
    }
}

impl<T: Notifier> Notifier for DesktopNotifier<T> {
    fn send(&self, message: &str) {
        Notification::new()
            .summary(&self.subject)
            .body(message)
            .show()
            .ok();

        if let Some(notifier) = &self.notifier {
            notifier.send(message);
        }
    }
}
