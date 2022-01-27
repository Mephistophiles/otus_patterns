use std::env;

mod decorator_dyn;
mod decorator_static;

fn main() {
    {
        use decorator_dyn::{BaseNotifier, DesktopNotifier, EmailNotifier, Notifier};
        // dynamic Decorator
        let mut notify: Box<dyn Notifier> = Box::new(BaseNotifier);

        if env::var("EMAIL").is_ok() {
            notify = EmailNotifier::new_notifier("to@email.com", "Notify testing", Some(notify));
        }

        if env::var("DESKTOP").is_ok() {
            notify = DesktopNotifier::new_notifier("Desktop notifications", Some(notify));
        }

        notify.send("Hello world");
    }

    {
        use decorator_static::{BaseNotifier, DesktopNotifier, EmailNotifier, Notifier};
        // static Decorator
        let notify = BaseNotifier;
        let notify = EmailNotifier::new("to@email.com", "Notify testing", Some(notify));
        let notify = DesktopNotifier::new("Desktop notifications", Some(notify));

        notify.send("Hello world");
    }
}
