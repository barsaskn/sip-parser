use crate::SipUri;

pub struct SipUser {
    uri: SipUri,
    tag: Option<String>,
}

impl SipUser {
    pub fn new(uri: SipUri, tag: Option<String>) -> SipUser {
        SipUser { uri, tag }
    }

    pub fn parse(user_str: &str) -> Option<SipUser> {
        let parts: Vec<&str> = user_str.split(';').collect();
        let mut tag: Option<String> = None;
        if parts.len() == 2 {
            let tag_part = parts[1].trim();
            tag = if tag_part.starts_with("tag=") {
                Some(tag_part[4..].to_string())
            } else {
                return None;
            };
        }

        let uri_part = parts[0].trim().trim_matches(|c| c == '<' || c == '>');
        let uri: SipUri = SipUri::parse(uri_part);

        return Some(SipUser { uri, tag });
    }

    // Getter methods
    pub fn get_uri(&self) -> &SipUri {
        &self.uri
    }

    pub fn get_tag(&self) -> &Option<String> {
        &self.tag
    }

    // Setter methods
    pub fn set_uri(&mut self, new_sip_uri: SipUri) {
        self.uri = new_sip_uri;
    }

    pub fn set_tag(&mut self, new_tag: Option<String>) {
        self.tag = new_tag;
    }
}

#[cfg(test)]
mod tests {
    use crate::SipUser;
    #[test]
    fn sip_user_parse_test() {
        let result = SipUser::parse("<sip:user@example.com:5060>;tag=5678").unwrap();
        let result_uri = result.get_uri();
        assert_eq!(result.get_tag().clone().unwrap(), "5678");
        assert_eq!(result_uri.get_ip(), "example.com");
        assert_eq!(result_uri.get_port(), 5060);
        assert_eq!(result_uri.get_username(), "user");
    }

    #[test]
    fn sip_user_parse_test_without_tag() {
        let result = SipUser::parse("<sip:user@example.com:5060>").unwrap();
        let result_uri = result.get_uri();
        assert_eq!(result.get_tag().clone(), None);
        assert_eq!(result_uri.get_ip(), "example.com");
        assert_eq!(result_uri.get_port(), 5060);
        assert_eq!(result_uri.get_username(), "user");
    }
}
