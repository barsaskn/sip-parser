#[derive(Debug)]
pub struct SipUri {
    ip: String,
    port: u16,
    username: String,
}

impl SipUri {
    pub fn new(ip: String, port: u16, username: String) -> SipUri {
        SipUri { ip, port, username }
    }

    pub fn parse(uri_str: &str) -> Result<SipUri, String> {
        if uri_str.is_empty() {
            return Err("[SipUri] Parsing error: Empty parameter".to_string());
        }
        let mut port: u16 = 5060;
        let parts: Vec<&str> = uri_str.split('@').collect();

        if parts.len() != 2 {
            return Err("[SipUri] Parsing error: Invalid format".to_string());
        }

        let mut username: String = parts[0].to_string();
        if username.starts_with("sip:") {
            username = username["sip:".len()..].to_string();
        }

        let address_parts: Vec<&str> = parts[1].split(':').collect();

        if address_parts.len() == 2 {
            port = address_parts[1].parse().expect("Invalid port number");
        }

        let ip: String = address_parts[0].to_string();

        Ok(SipUri { ip, port, username })
    }

    // Getter methods
    pub fn get_ip(&self) -> &String {
        &self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    // Setter methods
    pub fn set_ip(&mut self, new_ip: String) {
        self.ip = new_ip;
    }

    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }

    pub fn set_username(&mut self, new_username: String) {
        self.username = new_username;
    }
}

#[cfg(test)]
mod tests {
    use crate::SipUri;
    #[test]
    fn uri_parse_test() {
        let result = SipUri::parse("sip:test@example.com:5060").unwrap();
        assert_eq!(result.get_username(), "test");
        assert_eq!(result.get_ip(), "example.com");
        assert_eq!(result.get_port(), 5060);
    }

    #[test]
    fn uri_parse_test_without_port() {
        let result = SipUri::parse("sip:test@example.com").unwrap();
        assert_eq!(result.get_username(), "test");
        assert_eq!(result.get_ip(), "example.com");
        assert_eq!(result.get_port(), 5060);
    }

    #[test]
    fn uri_parse_test_empty_string() {
        let result = SipUri::parse("");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn uri_parse_test_random_string() {
        let result = SipUri::parse("Random string");
        assert_eq!(result.is_err(), true);
    }
}
