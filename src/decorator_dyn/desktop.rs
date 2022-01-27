use notify_rust::Notification;

use super::Notifier;

pub struct DesktopNotifier {
    subject: String,
    notifier: Option<Box<dyn Notifier>>,
}

impl DesktopNotifier {
    // TODO: clippy warns on new(): https://rust-lang.github.io/rust-clippy/master/index.html#new_ret_no_self
    pub fn new_notifier(
        subject: impl Into<String>,
        notifier: Option<Box<dyn Notifier>>,
    ) -> Box<dyn Notifier> {
        Box::new(Self {
            subject: subject.into(),
            notifier,
        })
    }
}

impl Notifier for DesktopNotifier {
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
