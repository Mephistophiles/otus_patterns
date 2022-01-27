#[derive(Debug, PartialEq)]
pub struct Host(String);
#[derive(Debug, PartialEq)]
pub struct Port(u16);

pub struct WebServer {
    pub host: Host,
    pub port: Port,
}

fn is_valid_part(part: &str) -> bool {
    if part.is_empty() || part.len() > 63 {
        return false;
    }

    part.chars()
        .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
}

impl TryFrom<String> for Host {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // https://datatracker.ietf.org/doc/html/rfc3696#section-2
        if value.is_empty() {
            return Err("Invalid hostname: empty string");
        }

        if value.len() > 255 {
            return Err("Invalid hostname: too big");
        }

        if value.starts_with('.') || value.starts_with('-') {
            return Err("Invalid hostname: wrong format");
        }

        let is_valid = value.split('.').all(is_valid_part);

        if !is_valid {
            return Err("Invalid hostname: wrong format");
        }

        Ok(Host(value))
    }
}

impl TryFrom<u16> for Port {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err("Invalid port: zero value");
        }

        Ok(Port(value))
    }
}

impl WebServer {
    pub fn new(host: Host, port: Port) -> Self {
        Self { host, port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host() {
        assert_eq!(
            String::from("localhost").try_into(),
            Ok(Host("localhost".to_string()))
        );

        assert_eq!(
            String::from("example.org").try_into(),
            Ok(Host("example.org".to_string()))
        );

        let host: Result<Host, _> = String::from("").try_into();
        assert_eq!(host, Err("Invalid hostname: empty string"));

        let host: Result<Host, _> = "l".repeat(256).try_into();
        assert_eq!(host, Err("Invalid hostname: too big"));

        let host: Result<Host, _> = ".example.org".to_string().try_into();
        assert_eq!(host, Err("Invalid hostname: wrong format"));

        let host: Result<Host, _> = "example:).org".to_string().try_into();
        assert_eq!(host, Err("Invalid hostname: wrong format"));
    }

    #[test]
    fn test_port() {
        assert_eq!(8080u16.try_into(), Ok(Port(8080)));

        let port: Result<Port, _> = 0.try_into();

        assert_eq!(port, Err("Invalid port: zero value"));
    }
}
