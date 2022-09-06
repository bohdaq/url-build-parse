use std::collections::HashMap;
use std::fmt::Error;

pub struct UrlComponents {
    pub scheme: String,
    pub authority: Authority,
    pub path: String,
    pub query: Option<HashMap<String, String>>,
    pub fragment: Option<String>
}

pub struct Authority {
    pub user_info: Option<UserInfo>,
    pub host: String,
    pub port: Option<String>
}

pub struct UserInfo {
    pub username: String,
    pub password: Option<String>
}

impl UrlComponents {
    pub fn new() -> UrlComponents {
        let url_components = UrlComponents {
            scheme: "".to_string(),
            authority: Authority {
                user_info: None,
                host: "".to_string(),
                port: None
            },
            path: "".to_string(),
            query: None,
            fragment: None
        };
        url_components
    }
}

pub fn parse_url(url: &str) -> UrlComponents {
    let mut url_components = UrlComponents::new();

    //let (scheme, remaining_url) = extract_scheme(url);

    url_components
}

pub(crate) fn extract_scheme(url: &str) -> Result<(String, String), String> {
    let boxed_scheme_and_remaining_url = url.split_once("://");
    if boxed_scheme_and_remaining_url.is_some() {
        let (scheme, remaining_url) = boxed_scheme_and_remaining_url.unwrap();
        return Ok((scheme.to_string(), remaining_url.to_string()))
    } else {
        return Err("unable to identify scheme".to_string());
    }
}




#[cfg(test)]
mod tests {
    use crate::{extract_scheme, parse_url};

    #[test]
    fn extract_scheme_test() {
        let url = "https://example.com";
        let boxed_result = extract_scheme(url);
        let (scheme, remaining_url) = boxed_result.unwrap();

        assert_eq!("https", scheme);
        assert_eq!("example.com", remaining_url);
    }

    #[test]
    fn parse_simple_url() {
        let url = "https://example.com";
        let url_components = parse_url(url);


        assert_eq!(url_components.scheme, "https");
        assert_eq!(url_components.authority.host, "example.com");
        assert_eq!(url_components.path, "");
    }
}
