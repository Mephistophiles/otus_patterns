pub mod desktop;
pub mod email;

pub use desktop::DesktopNotifier;
pub use email::EmailNotifier;

pub trait Notifier {
    fn send(&self, message: &str);
}

pub struct BaseNotifier;
impl Notifier for BaseNotifier {
    fn send(&self, _message: &str) {}
}
