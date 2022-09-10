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
    let boxed_split_at_path = url.split_once("://");
    if boxed_split_at_path.is_some() {
        let (scheme, remaining_url) = boxed_split_at_path.unwrap();
        Ok((scheme.to_string(), remaining_url.to_string()))
    } else {
        Err("unable to identify scheme".to_string())
    }
}

pub(crate) fn extract_authority(url: &str) -> Result<(String, Option<String>), String> {
    if url.chars().count() == 0 {
        return Err("unable to identify authority".to_string())
    }

    let mut is_there_a_slash = url.contains("/");
    let mut is_there_a_question_mark = url.contains("?");
    let mut is_there_a_hash = url.contains("#");

    if !is_there_a_slash && !is_there_a_question_mark && !is_there_a_hash {
        return Ok((url.to_string(), None))
    }

    if is_there_a_slash {
        let boxed_split = url.split_once("/");
        if boxed_split.is_some() {
            let (authority, remaining_url) = boxed_split.unwrap();
            let remaining_url = ["/", remaining_url].join("");
            return Ok((authority.to_string(), Option::from(remaining_url.to_string())))
        }
    }

    if !is_there_a_slash && is_there_a_question_mark {
        let boxed_split = url.split_once("?");
        if boxed_split.is_some() {
            let (authority, remaining_url) = boxed_split.unwrap();
            let remaining_url = ["?", remaining_url].join("");
            return Ok((authority.to_string(), Option::from(remaining_url.to_string())))
        }
    }

    if !is_there_a_slash && !is_there_a_question_mark && is_there_a_hash {
        let boxed_split = url.split_once("#");
        if boxed_split.is_some() {
            let (authority, remaining_url) = boxed_split.unwrap();
            let remaining_url = ["#", remaining_url].join("");
            return Ok((authority.to_string(), Option::from(remaining_url.to_string())))
        }
    }


    Err("not implemented yet".to_string())

}




#[cfg(test)]
mod tests {
    use crate::{extract_authority, extract_scheme, parse_url};

    #[test]
    fn extract_scheme_test() {
        let url = "https://example.com";
        let boxed_result = extract_scheme(url);
        let (scheme, remaining_url) = boxed_result.unwrap();

        assert_eq!("https", scheme);
        assert_eq!("example.com", remaining_url);
    }

    #[test]
    fn extract_authority_test() {
        let remaining_url = "example.com";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com", authority);
        assert_eq!(None, remaining_url);
    }

    #[test]
    fn extract_authority_path_defined_query_defined_fragment_defined() {
        let remaining_url = "example.com/some-path?q=test#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com", authority);
        assert_eq!("/some-path?q=test#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_path_defined_as_slash_query_defined_fragment_defined() {
        let remaining_url = "user:passwd@example.com:443/?q=test#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("user:passwd@example.com:443", authority);
        assert_eq!("/?q=test#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_path_undefined_query_defined_fragment_defined() {
        let remaining_url = "user:passwd@example.com?q=test#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("user:passwd@example.com", authority);
        assert_eq!("?q=test#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_path_undefined_query_undefined_fragment_defined() {
        let remaining_url = "example.com:80#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com:80", authority);
        assert_eq!("#123", remaining_url.unwrap());
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
