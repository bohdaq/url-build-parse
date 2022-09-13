use std::collections::HashMap;

pub struct UrlComponents {
    pub scheme: String,
    pub authority: Authority,
    pub path: Path,
    pub query: Option<HashMap<String, String>>,
    pub fragment: Option<String>
}

pub struct Authority {
    pub user_info: Option<UserInfo>,
    pub host: String,
    pub port: Option<usize>
}

pub struct UserInfo {
    pub username: String,
    pub password: Option<String>
}

pub struct Path {
    executable: String,
    path_info: Option<String>
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
            path: Path {
                executable: "".to_string(),
                path_info: None
            },
            query: None,
            fragment: None
        };
        url_components
    }
}

pub fn parse_url(url: &str) -> Result<UrlComponents, String> {
    let mut url_components = UrlComponents::new();
    let mut remaining_url = "".to_string();

    let boxed_scheme = extract_scheme(url);
    if boxed_scheme.is_err() {
        return Err(boxed_scheme.err().unwrap());
    }

    let (scheme, _remaining_url) = boxed_scheme.unwrap();
    url_components.scheme = scheme;
    remaining_url = _remaining_url;


    let boxed_authority = extract_authority(remaining_url.as_str());
    if boxed_authority.is_err() {
        return Err(boxed_authority.err().unwrap());
    }

    let (authority_string, boxed_remaining_url) = boxed_authority.unwrap();

    let boxed_authority = parse_authority(authority_string.as_str());
    if boxed_authority.is_err() {
        return Err(boxed_authority.err().unwrap());
    }

    let (boxed_userinfo, host, boxed_port) = boxed_authority.unwrap();
    if boxed_userinfo.is_some() {
        url_components.authority.user_info = boxed_userinfo;
    }
    url_components.authority.host = host;
    if boxed_port.is_some() {
        url_components.authority.port = boxed_port;
    }

    if boxed_remaining_url.is_some() {
        remaining_url = boxed_remaining_url.unwrap();
    } else {
        return Ok(url_components)
    }


    Ok(url_components)
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
        let error_message = "error: remaining url is empty";
        return Err(error_message.to_string())
    }

    let  is_there_a_slash = url.contains("/");
    let  is_there_a_question_mark = url.contains("?");
    let  is_there_a_hash = url.contains("#");

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

    let error_message = ["error: something went wrong with remaining url ", url].join("");
    Err(error_message.to_string())

}

pub(crate) fn extract_path(url: &str) -> Result<(String, Option<String>), String> {
    if url.chars().count() == 0 {
        let error_message = "error: remaining url is empty";
        return Err(error_message.to_string())
    }

    let is_there_a_slash = url.contains("/");
    let is_there_a_question_mark = url.contains("?");
    let is_there_a_hash = url.contains("#");

    if !is_there_a_slash && !is_there_a_question_mark && !is_there_a_hash {
        let error_message = ["error: not valid remaining url ", url].join("");
        return Err(error_message.to_string())
    }

    if is_there_a_slash {
        let boxed_split = url.split_once("/");
        if boxed_split.is_some() {
            let (_, path_query_url) = boxed_split.unwrap();
            let mut path = "".to_string();
            let mut remaining_url = "".to_string();

            if is_there_a_question_mark {
                let (_path, rest) = path_query_url.split_once("?").unwrap();
                path = _path.to_string();
                remaining_url = [&"?", rest].join("");
            }

            if !is_there_a_question_mark && is_there_a_hash {
                let (_path, rest) = path_query_url.split_once("#").unwrap();
                path = _path.to_string();
                remaining_url = [&"#", rest].join("");
            }

            if !is_there_a_question_mark && !is_there_a_hash {
                path = path_query_url.to_string();
            }

            let resulting_path = ["/".to_string(), path].join("");
            return Ok((resulting_path.to_string(), Option::from(remaining_url)))
        }
    }

    if !is_there_a_slash {
        return Ok(("".to_string(), Option::from(url.to_string())))
    }

    let error_message = ["error: something went wrong with remaining url ", url].join("");
    Err(error_message.to_string())

}

pub(crate) fn extract_query(url: &str) -> Result<(String, Option<String>), String> {
    if url.chars().count() == 0 {
        let error_message = "error: remaining url is empty";
        return Err(error_message.to_string())
    }

    let is_there_a_question_mark = url.contains("?");
    let is_there_a_hash = url.contains("#");

    if !is_there_a_question_mark {
        let error_message = ["error: query is not defined url: ", url].join("");
        return Err(error_message.to_string())
    }

    let (_, url) = url.split_once("?").unwrap();

    if is_there_a_hash {
        let (query, rest) = url.split_once("#").unwrap();
        let rest = ["#".to_string(), rest.to_string()].join("");
        Ok((query.to_string(), Option::from(rest.to_string())))
    } else {
        Ok((url.to_string(), None))
    }

}

pub(crate) fn extract_fragment(url: &str) -> Result<String, String> {
    if url.chars().count() == 0 {
        let error_message = "error: remaining url is empty";
        return Err(error_message.to_string())
    }

    let is_there_a_hash = url.contains("#");

    if !is_there_a_hash {
        let error_message = ["error: fragment is not defined url: ", url].join("");
        return Err(error_message.to_string())
    }

    let (_, fragment) = url.split_once("#").unwrap();

    let fragment = ["#".to_string(), fragment.to_string()].join("");
    Ok(fragment.to_string())

}

pub(crate) fn parse_authority(authority: &str) -> Result<(Option<UserInfo>, String, Option<usize>), String> {
    let mut user_info: UserInfo = UserInfo { username: "".to_string(), password: None };
    let mut host = "".to_string();
    let mut port : usize = 0;

    let mut remaining_authority = "".to_string();

    let is_there_an_at_symbol = authority.contains("@");
    if is_there_an_at_symbol {
        let (userinfo, _remaining_authority) = authority.split_once("@").unwrap();
        remaining_authority = _remaining_authority.to_string();
        let is_there_a_colon = userinfo.contains(":");
        if is_there_a_colon {
            let (username, password) = userinfo.split_once(":").unwrap();
            user_info.username = username.to_string();
            user_info.password = Some(password.to_string());
        } else {
            let username = userinfo.to_string();
            user_info.username = username;
        }
    }


    let is_there_a_colon = remaining_authority.contains(":");
    if is_there_a_colon {
        let (_host, port_as_string) = remaining_authority.split_once(":").unwrap();
        let boxed_port = port_as_string.parse::<usize>();
        if boxed_port.is_err() {
            let msg = [
                "unable to parse port from remaining authority ".to_string(),
                " | ".to_string(),
                boxed_port.err().unwrap().to_string(),
                " | ".to_string(),
                port_as_string.to_string()].join("");
            return Err(msg)
        }

        host = _host.to_string();
        port = boxed_port.unwrap();
    } else {
        host = remaining_authority;
    }

    Ok((Some(user_info), host, Some(port)))
}




#[cfg(test)]
mod tests {
    use crate::{extract_authority, extract_fragment, extract_path, extract_query, extract_scheme, parse_authority, parse_url};

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
    fn extract_authority_path_defined_query_undefined_fragment_defined() {
        let remaining_url = "example.com/some-path#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com", authority);
        assert_eq!("/some-path#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_undefined_path_zero_length_query_undefined_fragment_undefined() {
        let remaining_url = "";
        let boxed_result = extract_authority(remaining_url);
        assert!(boxed_result.is_err());
        assert_eq!("error: remaining url is empty", boxed_result.err().unwrap());
    }

    #[test]
    fn extract_authority_defined_path_zero_length_query_undefined_fragment_undefined() {
        let remaining_url = "usr:pwd@host:443";
        let boxed_result = extract_authority(remaining_url);

        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("usr:pwd@host:443", authority);
        assert_eq!(None, remaining_url);
    }

    #[test]
    fn extract_path_path_defined_query_undefined_fragment_defined() {
        let remaining_url = "/some-path#123";
        let boxed_result = extract_path(remaining_url);
        let (path, remaining_url) = boxed_result.unwrap();

        assert_eq!("/some-path", path);
        assert_eq!("#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_path_path_defined_query_defined_fragment_defined() {
        let remaining_url = "/some-path?q=query#123";
        let boxed_result = extract_path(remaining_url);
        let (path, remaining_url) = boxed_result.unwrap();

        assert_eq!("/some-path", path);
        assert_eq!("?q=query#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_path_path_defined_query_defined_fragment_undefined() {
        let remaining_url = "/some-path?q=query";
        let boxed_result = extract_path(remaining_url);
        let (path, remaining_url) = boxed_result.unwrap();

        assert_eq!("/some-path", path);
        assert_eq!("?q=query", remaining_url.unwrap());
    }

    #[test]
    fn extract_path_path_defined_as_slash_query_defined_fragment_undefined() {
        let remaining_url = "/?q=query";
        let boxed_result = extract_path(remaining_url);
        let (path, remaining_url) = boxed_result.unwrap();

        assert_eq!("/", path);
        assert_eq!("?q=query", remaining_url.unwrap());
    }

    #[test]
    fn extract_path_path_zero_length_query_defined_fragment_defined() {
        let remaining_url = "?q=query#fragment";
        let boxed_result = extract_path(remaining_url);
        let (path, remaining_url) = boxed_result.unwrap();

        assert_eq!("", path);
        assert_eq!("?q=query#fragment", remaining_url.unwrap());
    }

    #[test]
    fn extract_path_path_zero_length_query_undefined_fragment_defined() {
        let remaining_url = "#fragment";
        let boxed_result = extract_path(remaining_url);
        let (path, remaining_url) = boxed_result.unwrap();

        assert_eq!("", path);
        assert_eq!("#fragment", remaining_url.unwrap());
    }

    #[test]
    fn extract_path_path_zero_length_query_defined_fragment_undefined() {
        let remaining_url = "?q=query";
        let boxed_result = extract_path(remaining_url);
        let (path, remaining_url) = boxed_result.unwrap();

        assert_eq!("", path);
        assert_eq!("?q=query", remaining_url.unwrap());
    }

    #[test]
    fn extract_path_path_zero_length_query_undefined_fragment_undefined() {
        let remaining_url = "";
        let boxed_result = extract_path(remaining_url);
        assert!(boxed_result.is_err());
        assert_eq!("error: remaining url is empty", boxed_result.err().unwrap());
    }

    #[test]
    fn extract_query_empty_remaining_url() {
        let remaining_url = "";
        let boxed_result = extract_query(remaining_url);
        assert!(boxed_result.is_err());
        assert_eq!("error: remaining url is empty", boxed_result.err().unwrap());
    }

    #[test]
    fn extract_query_query_undefined() {
        let remaining_url = "sometext#qweqwe";
        let boxed_result = extract_query(remaining_url);
        assert!(boxed_result.is_err());
        assert_eq!("error: query is not defined url: sometext#qweqwe", boxed_result.err().unwrap());
    }

    #[test]
    fn extract_query_query_defined_fragment_undefined() {
        let remaining_url = "?q=query";
        let boxed_result = extract_query(remaining_url);
        let (query, remaining_url) = boxed_result.unwrap();

        assert_eq!("q=query", query);
        assert_eq!(None, remaining_url);
    }

    #[test]
    fn extract_query_query_defined_fragment_defined() {
        let remaining_url = "?q=query#fragment1";
        let boxed_result = extract_query(remaining_url);
        let (query, remaining_url) = boxed_result.unwrap();

        assert_eq!("q=query", query);
        assert_eq!("#fragment1", remaining_url.unwrap());
    }

    #[test]
    fn extract_fragment_undefined() {
        let remaining_url = "gment1";
        let boxed_result = extract_fragment(remaining_url);
        assert!(boxed_result.is_err());
        assert_eq!("error: fragment is not defined url: gment1", boxed_result.err().unwrap());
    }

    #[test]
    fn extract_fragment_undefined_empty() {
        let remaining_url = "";
        let boxed_result = extract_fragment(remaining_url);
        assert!(boxed_result.is_err());
        assert_eq!("error: remaining url is empty", boxed_result.err().unwrap());
    }

    #[test]
    fn extract_fragment_defined() {
        let remaining_url = "#test";
        let boxed_result = extract_fragment(remaining_url);
        assert!(boxed_result.is_ok());
        assert_eq!("#test", boxed_result.unwrap());
    }

    #[test]
    fn parse_authority_parts() {
        let authority = "usr:pwd@somehost:80";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());
        let (boxed_user_info, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_user_info.is_some());
        let user_info = boxed_user_info.unwrap();

        assert_eq!("usr", user_info.username);

        assert!(user_info.password.is_some());
        assert_eq!("pwd", user_info.password.unwrap());

        assert_eq!("somehost", host);

        assert!(boxed_port.is_some());
        assert_eq!(80, boxed_port.unwrap());
    }

    #[test]
    fn parse_simple_url() {
        let url = "https://usr:pwd@somehost:80";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "https");
        assert_eq!(url_components.authority.user_info.as_ref().unwrap().username, "usr");
        assert_eq!(url_components.authority.user_info.as_ref().unwrap().password.as_ref().unwrap(), "pwd");
        assert_eq!(url_components.authority.host, "somehost");
        assert_eq!(url_components.path.executable, "");

        assert!(false)
    }
}
