pub struct WebServer {
    pub host: String,
    pub port: u16,
}

#[derive(Default)]
pub struct WebServerBuilder {
    host: Option<String>,
    port: Option<u16>,
}

impl WebServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    // self - for reduce memory allocations (reduce clone() count)
    pub fn build(self) -> WebServer {
        WebServer {
            host: self.host.unwrap_or_else(|| "localhost".to_owned()),
            port: self.port.unwrap_or(8080),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_args() {
        let server = WebServerBuilder::new().build();

        assert_eq!(&server.host, "localhost");
        assert_eq!(server.port, 8080);
    }

    #[test]
    fn redefine_host() {
        let server = WebServerBuilder::new().host("example.org").build();

        assert_eq!(&server.host, "example.org");
        assert_eq!(server.port, 8080);
    }

    #[test]
    fn redefine_port() {
        let server = WebServerBuilder::new().port(80).build();
        assert_eq!(&server.host, "localhost");
        assert_eq!(server.port, 80);
    }

    #[test]
    fn redefine_host_and_port() {
        let server = WebServerBuilder::new().host("example.org").port(80).build();
        assert_eq!(&server.host, "example.org");
        assert_eq!(server.port, 80);
    }
}
