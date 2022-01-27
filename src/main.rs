use std::env;

mod builder_pattern;
mod decorator_dyn;
mod decorator_static;
mod newtype_pattern;

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

    // 23_patterns
    {
        let _server = builder_pattern::WebServerBuilder::new()
            .host("localhost")
            .port(8080)
            .build();
    }

    {
        use newtype_pattern::WebServer;

        let host = "localhost".to_string().try_into().unwrap();
        let port = 8080.try_into().unwrap();
        let _server = WebServer::new(host, port);
    }
}
