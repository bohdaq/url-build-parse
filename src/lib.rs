use std::collections::HashMap;
use url_search_params;
use url_search_params::{build_url_search_params, parse_url_search_params};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UrlComponents {
    pub scheme: String,
    pub authority: Option<UrlAuthority>,
    pub path: String,
    pub query: Option<HashMap<String, String>>,
    pub fragment: Option<String>
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UrlAuthority {
    pub user_info: Option<UrlUserInfo>,
    pub host: String,
    pub port: Option<usize>
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UrlUserInfo {
    pub username: String,
    pub password: Option<String>
}


impl UrlComponents {
    pub fn new() -> UrlComponents {
        let url_components = UrlComponents {
            scheme: "".to_string(),
            authority: None,
            path: "".to_string(),
            query: None,
            fragment: None };
        url_components
    }
}

pub fn parse_url(url: &str) -> Result<UrlComponents, String> {
    let mut url_components = UrlComponents::new();

    let boxed_scheme = extract_scheme(url);
    if boxed_scheme.is_err() {
        return Err(boxed_scheme.err().unwrap());
    }

    let (scheme, _remaining_url) = boxed_scheme.unwrap();
    url_components.scheme = scheme;
    let mut remaining_url = _remaining_url;


    let boxed_authority = extract_authority(remaining_url.as_str());
    if boxed_authority.is_err() {
        return Err(boxed_authority.err().unwrap());
    }

    let (authority_string, boxed_remaining_url) = boxed_authority.unwrap();

    if authority_string.is_some() {
        let boxed_authority = parse_authority(authority_string.unwrap().as_str());
        if boxed_authority.is_err() {
            return Err(boxed_authority.err().unwrap());
        }

        let (boxed_username, boxed_password, host, boxed_port) = boxed_authority.unwrap();


        let mut authority = UrlAuthority {
            user_info: None,
            host,
            port: None
        };

        let mut user_info : Option<UrlUserInfo> = None;

        if boxed_username.is_some() {
            let username = boxed_username.unwrap();
            if user_info.is_none() {
                let mut password : Option<String> = None;
                if boxed_password.is_some() {
                    password = boxed_password;
                }
                user_info = Some(UrlUserInfo { username: username.to_string(), password });
            }

            authority.user_info = user_info;
        }


        if boxed_port.is_some() {
            let port = boxed_port;
            authority.port = port;
        }

        url_components.authority = Option::from(authority);
    }



    if boxed_remaining_url.is_none() {
        return Ok(url_components)
    }
    remaining_url = boxed_remaining_url.unwrap();

    let boxed_path = extract_path(remaining_url.as_str());
    if boxed_path.is_err() {
        return Err(boxed_path.err().unwrap());
    }
    let (_path, _remaining_url) = boxed_path.unwrap();

    url_components.path = _path;
    if _remaining_url.is_none() {
        return Ok(url_components)
    }
    remaining_url = _remaining_url.unwrap();


    let (boxed_query, _remaining_url) = extract_query(remaining_url.as_str());
    if boxed_query.is_some() {
        let query  = boxed_query.unwrap();
        let parsed_query = parse_query(query.as_str()).unwrap();
        let params: HashMap<String, String> = parse_url_search_params(parsed_query.as_str());
        url_components.query = Some(params);
        if _remaining_url.is_none() {
            return Ok(url_components)
        }
        remaining_url = _remaining_url.unwrap();
    }

    let boxed_fragment = extract_fragment(remaining_url.as_str());
    if boxed_fragment.is_err() {
        return Err(boxed_fragment.err().unwrap());
    }

    let fragment = parse_fragment(boxed_fragment.unwrap().as_str()).unwrap();
    url_components.fragment = Option::from(fragment);

    Ok(url_components)
}

pub fn build_url(url_components: UrlComponents) -> Result<String, String> {
    let mut url = "".to_string();

    if url_components.fragment.is_some() {
        url = ["#".to_string(),  url_components.fragment.unwrap(), url].join("");
    }

    if url_components.query.is_some() {
        let query = build_url_search_params(url_components.query.unwrap());
        url = ["?".to_string(), query, url].join("");
    }

    url = [url_components.path, url].join("");

    if url_components.authority.is_some() {
        let authority = build_authority(url_components.authority.unwrap());
        url = ["//".to_string(), authority, url].join("");
    }

    url = [url_components.scheme, ":".to_string(), url].join("");

    Ok(url)
}

pub(crate) fn build_authority(url_authority: UrlAuthority) -> String {
    let mut authority = "".to_string();

    if url_authority.user_info.is_some() {
        let url_user_info = url_authority.user_info.unwrap();
        authority = url_user_info.username;
        if url_user_info.password.is_some() {
            authority = [authority, ":".to_string(), url_user_info.password.unwrap()].join("");
        }
    }

    if authority.chars().count() != 0 {
        authority = [authority, "@".to_string(), url_authority.host].join("");
    }

    if url_authority.port.is_some() {
        authority = [authority, ":".to_string(), url_authority.port.unwrap().to_string()].join("");
    }

    authority
}

pub(crate) fn extract_scheme(url: &str) -> Result<(String, String), String> {
    let boxed_split_at_path = url.split_once(":");
    if boxed_split_at_path.is_some() {
        let (scheme, remaining_url) = boxed_split_at_path.unwrap();
        Ok((scheme.to_string(), remaining_url.to_string()))
    } else {
        Err("unable to identify scheme".to_string())
    }
}

pub(crate) fn extract_authority(mut url: &str) -> Result<(Option<String>, Option<String>), String> {
    if url.chars().count() == 0 {
        let error_message = "error: remaining url is empty";
        return Err(error_message.to_string())
    }

    if !url.contains("//") {
        return Ok((None, Option::from(url.to_string())))
    }

    let (_, _remaining_url) = url.split_once("//").unwrap();
    url = _remaining_url;

    let  is_there_a_slash = url.contains("/");
    let  is_there_a_question_mark = url.contains("?");
    let  is_there_a_hash = url.contains("#");

    if !is_there_a_slash && !is_there_a_question_mark && !is_there_a_hash {
        return Ok((Option::from(url.to_string()), None))
    }

    if is_there_a_slash {
        let boxed_split = url.split_once("/");
        if boxed_split.is_some() {
            let (authority, remaining_url) = boxed_split.unwrap();
            let remaining_url = ["/", remaining_url].join("");
            let authority_option = Option::from(authority.to_string());
            let remaining_url = Option::from(remaining_url.to_string());
            return Ok((authority_option, remaining_url))
        }
    }

    if !is_there_a_slash && is_there_a_question_mark {
        let boxed_split = url.split_once("?");
        if boxed_split.is_some() {
            let (authority, remaining_url) = boxed_split.unwrap();
            let authority_option = Option::from(authority.to_string());
            let remaining_url = ["?", remaining_url].join("");
            let remaining_url = Option::from(remaining_url.to_string());
            return Ok((authority_option, remaining_url))
        }
    }

    if !is_there_a_slash && !is_there_a_question_mark && is_there_a_hash {
        let boxed_split = url.split_once("#");
        if boxed_split.is_some() {
            let (authority, remaining_url) = boxed_split.unwrap();
            let remaining_url = ["#", remaining_url].join("");
            let authority_option = Option::from(authority.to_string());
            let remaining_url = Option::from(remaining_url.to_string());
            return Ok((authority_option, remaining_url))
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

    let is_there_a_question_mark = url.contains("?");
    let is_there_a_hash = url.contains("#");

    if !is_there_a_question_mark && !is_there_a_hash {
        return Ok((url.to_string(), None));
    }

    let mut delimiter = "?";
    if !is_there_a_question_mark && is_there_a_hash {
        delimiter = "#";
    }

    let boxed_split = url.split_once(&delimiter);
    if boxed_split.is_some() {
        let (_path, _rest) = boxed_split.unwrap();
        let path = _path.to_string();
        let remaining_url: String =
            [delimiter.to_string(), _rest.to_string()].join("");

        return Ok((path.to_string(), Option::from(remaining_url)));
    }


    let error_message = ["error: something went wrong with remaining url ", url].join("");
    Err(error_message.to_string())

}

pub(crate) fn extract_query(url: &str) ->
       (Option<String>, Option<String>) {
    if url.chars().count() == 0 {
        return (None, None);
    }

    let is_there_a_hash = url.contains("#");

    if is_there_a_hash {
        let (query, rest) = url.split_once("#").unwrap();
        let rest = ["#".to_string(), rest.to_string()].join("");
        let mut query_option : Option<String> = None;

        if query.chars().count() != 0 {
            query_option = Some(query.to_string());
        }

        (query_option, Option::from(rest.to_string()))
    } else {
        (Option::from(url.to_string()), None)
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

pub(crate) fn parse_query(query_with_question_mark: &str) -> Result<String, String> {
    let (_, query) = query_with_question_mark.split_once("?").unwrap();

    Ok(query.to_string())
}

pub(crate) fn parse_fragment(url: &str) -> Result<String, String> {
    let (_, fragment) = url.split_once("#").unwrap();

    Ok(fragment.to_string())
}

pub(crate) fn parse_authority(authority: &str)
    -> Result<
        (
            Option<String>,
            Option<String>,
            String,
            Option<usize>
        ), String> {
    let mut port : Option<usize> = None;

    let mut remaining_authority = authority.to_string();

    let boxed_userinfo = extract_userinfo(remaining_authority.as_str());
    let (username, password, _remaining_authority) = boxed_userinfo.unwrap();
    remaining_authority = _remaining_authority;

    let boxed_host = extract_host(remaining_authority.as_str());
    let (host, _remaining_authority) = boxed_host.unwrap();

    if _remaining_authority.is_some() {
        let boxed_port = extract_port(_remaining_authority.unwrap().as_str());
        port = boxed_port.unwrap();
    }

    Ok((username, password, host, port))
}

pub(crate) fn extract_userinfo(authority: &str) -> Result<(Option<String>, Option<String>, String), String> {
    let mut username : Option<String> = None;
    let mut password : Option<String> = None;


    let mut remaining_authority = authority.to_string();

    let is_there_an_at_symbol = authority.contains("@");
    if is_there_an_at_symbol {
        let (userinfo, _remaining_authority) = authority.split_once("@").unwrap();
        remaining_authority = _remaining_authority.to_string();
        let is_there_a_colon = userinfo.contains(":");
        if is_there_a_colon {
            let (_username, _password) = userinfo.split_once(":").unwrap();
            username = Some(_username.to_string());
            password = Some(_password.to_string());
        } else {
            let _username = userinfo.to_string();
            username = Some(_username);
        }
    }

    Ok((username, password, remaining_authority))
}

pub(crate) fn extract_host(authority: &str) -> Result<(String, Option<String>), String> {
    let mut host : String = authority.to_string();
    let mut remaining_authority: Option<String> = None;

    let is_it_an_ip_v6_url = authority.contains("]");
    if is_it_an_ip_v6_url {
        let (_host, _remaining_authority) = authority.split_once("]").unwrap();
        host = [_host, "]"].join("");
        let it_contains_port = _remaining_authority.contains(":");
        if it_contains_port {
            remaining_authority = Option::from(_remaining_authority.to_string());
        }
    } else {
        let it_contains_port = authority.contains(":");
        if it_contains_port {
            let (_host, _remaining_authority) = authority.split_once(":").unwrap();
            host = _host.to_string();
            remaining_authority = Option::from([":", _remaining_authority].join(""));
        }
    }

    Ok((host, remaining_authority))
}

pub(crate) fn extract_port(authority: &str) -> Result<Option<usize>, String> {
    let mut port: Option<usize> = None;

    let is_there_a_colon = authority.contains(":");
    if is_there_a_colon {
        let (_, port_as_string) = authority.split_once(":").unwrap();

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

        port = Some(boxed_port.unwrap());
    }

    Ok(port)
}




#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{build_authority, build_url, extract_authority, extract_fragment, extract_host, extract_path, extract_port, extract_query, extract_scheme, extract_userinfo, parse_authority, parse_url, UrlAuthority, UrlComponents, UrlUserInfo};

    #[test]
    fn extract_scheme_test_no_delimiter() {
        let url = "schemewithoutdelimiter";
        let boxed_result = extract_scheme(url);

        assert!(boxed_result.is_err());
        assert_eq!("unable to identify scheme", boxed_result.err().unwrap());
    }

    #[test]
    fn extract_scheme_test() {
        let url = "https://example.com";
        let boxed_result = extract_scheme(url);
        let (scheme, remaining_url) = boxed_result.unwrap();

        assert_eq!("https", scheme);
        assert_eq!("//example.com", remaining_url);
    }

    #[test]
    fn extract_authority_test_no_authority() {
        let remaining_url = "/path?q=qwerty";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!(None, authority);
        assert_eq!("/path?q=qwerty", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_test_no_authority_no_slash() {
        let remaining_url = "path?q=qwerty";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!(None, authority);
        assert_eq!("path?q=qwerty", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_test() {
        let remaining_url = "//example.com";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com", authority.unwrap());
        assert_eq!(None, remaining_url);
    }

    #[test]
    fn extract_authority_path_defined_query_defined_fragment_defined() {
        let remaining_url = "//example.com/some-path?q=test#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com", authority.unwrap());
        assert_eq!("/some-path?q=test#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_path_defined_as_slash_query_defined_fragment_defined() {
        let remaining_url = "//user:passwd@example.com:443/?q=test#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("user:passwd@example.com:443", authority.unwrap());
        assert_eq!("/?q=test#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_path_undefined_query_defined_fragment_defined() {
        let remaining_url = "//user:passwd@example.com?q=test#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("user:passwd@example.com", authority.unwrap());
        assert_eq!("?q=test#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_path_undefined_query_undefined_fragment_defined() {
        let remaining_url = "//example.com:80#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com:80", authority.unwrap());
        assert_eq!("#123", remaining_url.unwrap());
    }

    #[test]
    fn extract_authority_path_defined_query_undefined_fragment_defined() {
        let remaining_url = "//example.com/some-path#123";
        let boxed_result = extract_authority(remaining_url);
        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("example.com", authority.unwrap());
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
        let remaining_url = "//usr:pwd@host:443";
        let boxed_result = extract_authority(remaining_url);

        let (authority, remaining_url) = boxed_result.unwrap();

        assert_eq!("usr:pwd@host:443", authority.unwrap());
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
        let (boxed_query, _remaining_url) = extract_query(remaining_url);
        assert!(boxed_query.is_none());
    }

    #[test]
    fn extract_query_query_undefined() {
        let remaining_url = "#qweqwe";
        let (query, remaining_url) = extract_query(remaining_url);

        assert!(query.is_none());
        assert_eq!("#qweqwe", remaining_url.unwrap());
    }

    #[test]
    fn extract_query_query_defined_fragment_undefined() {
        let remaining_url = "?q=query";
        let (query, _remaining_url) = extract_query(remaining_url);

        assert_eq!("?q=query", query.unwrap());
        assert_eq!(None, _remaining_url);
    }

    #[test]
    fn extract_query_query_defined_fragment_defined() {
        let remaining_url = "?q=query#fragment1";
        let (query, remaining_url) = extract_query(remaining_url);

        assert_eq!("?q=query", query.unwrap());
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
        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_some());
        assert_eq!("usr", boxed_username.unwrap());

        assert!(boxed_password.is_some());
        assert_eq!("pwd", boxed_password.unwrap());

        assert_eq!("somehost", host);

        assert!(boxed_port.is_some());
        assert_eq!(80, boxed_port.unwrap());
    }

    #[test]
    fn parse_authority_parts_no_password() {
        let authority = "usr@somehost:80";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());
        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_some());
        assert_eq!("usr", boxed_username.unwrap());

        assert!(boxed_password.is_none());

        assert_eq!("somehost", host);

        assert!(boxed_port.is_some());
        assert_eq!(80, boxed_port.unwrap());
    }

    #[test]
    fn parse_authority_parts_no_user_no_password() {
        let authority = "somehost:80";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());
        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_none());
        assert!(boxed_password.is_none());

        assert_eq!("somehost", host);

        assert!(boxed_port.is_some());
        assert_eq!(80, boxed_port.unwrap());
    }

    #[test]
    fn parse_authority_parts_no_user_no_password_no_port() {
        let authority = "somehost";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());
        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_none());
        assert!(boxed_password.is_none());

        assert_eq!("somehost", host);

        assert!(boxed_port.is_none());
    }

    #[test]
    fn parse_authority_parts_no_password_no_port() {
        let authority = "usr@somehost";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());
        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_some());
        assert_eq!("usr", boxed_username.unwrap());
        assert!(boxed_password.is_none());

        assert_eq!("somehost", host);

        assert!(boxed_port.is_none());
    }


    #[test]
    fn parse_authority_parts_no_port() {
        let authority = "usr:pwd@somehost";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());
        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_some());
        assert_eq!("usr", boxed_username.unwrap());


        assert!(boxed_password.is_some());
        assert_eq!("pwd", boxed_password.unwrap());

        assert_eq!("somehost", host);

        assert!(boxed_port.is_none());
    }

    #[test]
    fn parse_extract_userinfo() {
        let boxed_userinfo =
            extract_userinfo(
                "usr:pwd@[2001:0db8:85a3:0000:0000:8a2e:0370:7334]");
        assert!(boxed_userinfo.is_ok());

        let (username, password, remaining_authority) = boxed_userinfo.unwrap();

        assert_eq!("usr", username.unwrap());
        assert_eq!("pwd", password.unwrap());
        assert_eq!("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]", remaining_authority);
    }


    #[test]
    fn parse_extract_userinfo_no_passwd() {
        let boxed_userinfo =
            extract_userinfo(
                "usr@192.168.0.1");
        assert!(boxed_userinfo.is_ok());

        let (username, password, remaining_authority) = boxed_userinfo.unwrap();

        assert_eq!("usr", username.unwrap());
        assert_eq!(None, password);
        assert_eq!("192.168.0.1", remaining_authority);
    }

    #[test]
    fn parse_extract_userinfo_no_passwd_no_user() {
        let boxed_userinfo =
            extract_userinfo(
                "somehost.com");
        assert!(boxed_userinfo.is_ok());

        let (username, password, remaining_authority) = boxed_userinfo.unwrap();

        assert_eq!(None, username);
        assert_eq!(None, password);
        assert_eq!("somehost.com", remaining_authority);
    }

    #[test]
    fn parse_extract_host_ip_v4() {
        let (host, remaining_authority) =
            extract_host("somehost.com:80".as_ref()).unwrap();

        assert_eq!("somehost.com", host);
        assert_eq!(":80", remaining_authority.unwrap());
    }

    #[test]
    fn parse_extract_host_ip_v4_no_port() {
        let (host, remaining_authority) =
            extract_host("somehost.com".as_ref()).unwrap();

        assert_eq!("somehost.com", host);
        assert_eq!(None, remaining_authority);
    }


    #[test]
    fn parse_extract_host_ip_v6() {
        let (host, remaining_authority) =
            extract_host("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:80".as_ref()).unwrap();

        assert_eq!("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]", host);
        assert_eq!(":80", remaining_authority.unwrap());
    }

    #[test]
    fn parse_extract_host_ip_v6_no_port() {
        let (host, remaining_authority) =
            extract_host("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]".as_ref()).unwrap();

        assert_eq!("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]", host);
        assert_eq!(None, remaining_authority);
    }

    #[test]
    fn parse_authority_parts_ip_v6() {
        let authority = "[2001:0db8:85a3:0000:0000:8a2e:0370:7334]";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());

        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_none());


        assert!(boxed_password.is_none());

        assert_eq!("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]", host);

        assert!(boxed_port.is_none());
    }

    #[test]
    fn parse_authority_parts_usr_pwd_ip_v6_port() {
        let authority = "usr:pwd@[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:80";
        let boxed_result = parse_authority(authority);


        assert!(boxed_result.is_ok());
        let (boxed_username, boxed_password, host, boxed_port) = boxed_result.unwrap();

        assert!(boxed_username.is_some());
        assert_eq!("usr", boxed_username.unwrap());

        assert!(boxed_password.is_some());
        assert_eq!("pwd", boxed_password.unwrap());

        assert_eq!("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]", host);

        assert!(boxed_port.is_some());
        assert_eq!(80, boxed_port.unwrap());
    }

    #[test]
    fn parse_simple_url_no_authority() {
        let url = "mailto:user@host,user2@host";

        let url_components = parse_url(url).unwrap();

        assert_eq!(url_components.scheme, "mailto");
        assert!(url_components.authority.is_none());
        assert_eq!(url_components.path, "user@host,user2@host");

    }

    #[test]
    fn parse_simple_url_no_authority_with_query() {
        let url = "mailto:user@host?subject=test#fragment";

        let url_components = parse_url(url).unwrap();

        assert_eq!(url_components.scheme, "mailto");
        assert!(url_components.authority.is_none());
        assert_eq!(url_components.path, "user@host");
        assert_eq!(url_components.fragment.unwrap(), "fragment");

    }

    #[test]
    fn parse_simple_url_no_authority_with_fragment() {
        let url = "mailto:user@host#fragment";

        let url_components = parse_url(url).unwrap();

        assert_eq!(url_components.scheme, "mailto");
        assert!(url_components.authority.is_none());
        assert_eq!(url_components.path, "user@host");
        assert_eq!(url_components.fragment.unwrap(), "fragment");

    }

    #[test]
    fn parse_simple_url_no_authority_with_query_with_fragment() {
        let url = "mailto:user@host?q=123#fragment";

        let url_components = parse_url(url).unwrap();

        assert_eq!(url_components.scheme, "mailto");
        assert!(url_components.authority.is_none());
        assert_eq!(url_components.path, "user@host");

    }

    #[test]
    fn parse_simple_url_no_authority_no_path_with_query_with_fragment() {
        let url = "mailto:?to=&subject=mailto%20with%20examples&body=https%3A%2F%2Fen.wikipedia.org%2Fwiki%2FMailto";

        let url_components = parse_url(url).unwrap();

        assert_eq!(url_components.scheme, "mailto");
        assert!(url_components.authority.is_none());
        assert_eq!(url_components.path, "");
        assert_eq!(url_components.query.as_ref().unwrap().get("subject").unwrap(), "mailto%20with%20examples");
        assert_eq!(url_components.query.as_ref().unwrap().get("to").unwrap(), "");
        assert_eq!(url_components.query.as_ref().unwrap().get("body").unwrap(), "https%3A%2F%2Fen.wikipedia.org%2Fwiki%2FMailto");

    }

    #[test]
    fn extract_port_test() {
        let boxed_port = extract_port(":80");
        assert!(boxed_port.is_ok());
        assert_eq!(80, boxed_port.unwrap().unwrap());
    }

    #[test]
    fn extract_port_test_fail() {
        let boxed_port = extract_port(":someport");
        assert!(boxed_port.is_err());
        assert_eq!("unable to parse port from remaining authority  | invalid digit found in string | someport", boxed_port.err().unwrap());
    }


    #[test]
    fn build_authority_host_empty() {
        let authority = UrlAuthority{
            user_info: None,
            host: "".to_string(),
            port: None
        };

        let url_authority = build_authority(authority);

        assert_eq!(url_authority, "");
    }

    #[test]
    fn build_authority_host_empty_usrname() {
        let authority = UrlAuthority{
            user_info: Option::from(UrlUserInfo { username: "usr".to_string(), password: None }),
            host: "".to_string(),
            port: None
        };

        let url_authority = build_authority(authority);

        assert_eq!(url_authority, "usr@");
    }

    #[test]
    fn build_authority_host_usrname_passwd() {
        let authority = UrlAuthority{
            user_info: Option::from(UrlUserInfo { username: "usr".to_string(), password: Option::from("pwd".to_string()) }),
            host: "somehost".to_string(),
            port: None
        };

        let url_authority = build_authority(authority);

        assert_eq!(url_authority, "usr:pwd@somehost");
    }

    #[test]
    fn build_url_all_specified() {
        let authority = UrlAuthority{
            user_info: Option::from(UrlUserInfo { username: "usr".to_string(), password: Option::from("pwd".to_string()) }),
            host: "somehost".to_string(),
            port: Option::from(80)
        };

        let mut q = HashMap::new();
        q.insert("q".to_string(), "123".to_string());
        q.insert("w".to_string(), "456".to_string());


        let url_components = UrlComponents{
            scheme: "https".to_string(),
            authority: Option::from(authority),
            path: "/".to_string(),
            query: Option::from(q),
            fragment: Option::from("fragment".to_string())
        };

        let url = build_url(url_components.clone()).unwrap();

        let parsed_url_components = parse_url(url.as_str()).unwrap();

        assert_eq!(url_components, parsed_url_components);
    }

    #[test]
    fn build_url_only_required_specified() {
        let url_components = UrlComponents{
            scheme: "https".to_string(),
            authority: None,
            path: "/".to_string(),
            query: None,
            fragment: None
        };

        let url = build_url(url_components.clone()).unwrap();
        let parsed_url_components = parse_url(url.as_str()).unwrap();
        assert_eq!(url_components, parsed_url_components);
    }

    #[test]
    fn build_authority_host_usrname_passwd_port() {
        let authority = UrlAuthority{
            user_info: Option::from(UrlUserInfo { username: "usr".to_string(), password: Option::from("pwd".to_string()) }),
            host: "somehost".to_string(),
            port: Option::from(80)
        };

        let url_authority = build_authority(authority);

        assert_eq!(url_authority, "usr:pwd@somehost:80");
    }

    #[test]
    fn parse_simple_url_no_path_no_query_no_fragment() {
        let url = "https://usr:pwd@somehost:80";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "https");
        assert_eq!(url_components.authority.as_ref().unwrap().user_info.as_ref().unwrap().username, "usr");
        assert_eq!(url_components.authority.as_ref().unwrap().user_info.as_ref().unwrap().password.as_ref().unwrap(), "pwd");
        assert_eq!(url_components.authority.as_ref().unwrap().host, "somehost");
        assert_eq!(*url_components.authority.as_ref().unwrap().port.as_ref().unwrap() as u8, 80 as u8);
        assert_eq!(url_components.path, "");

    }

    #[test]
    fn parse_simple_url_no_usr_no_pwd_no_path_no_query_no_fragment() {
        let url = "https://somehost";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "https");
        assert!(url_components.authority.as_ref().unwrap().user_info.is_none());
        assert!(url_components.authority.as_ref().unwrap().port.is_none());
        assert_eq!(url_components.authority.as_ref().unwrap().host, "somehost");
        assert_eq!(url_components.path, "");
        assert_eq!(url_components.query, None);
        assert_eq!(url_components.fragment, None);

    }

    #[test]
    fn parse_simple_url() {
        let url = "https://usr:pwd@somehost:80/path?param=value&anotherParam#fragment";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "https");
        assert_eq!(url_components.authority.as_ref().unwrap().user_info.as_ref().unwrap()
                       .username, "usr");
        assert_eq!(url_components.authority.as_ref().unwrap().user_info.as_ref().unwrap()
                       .password.as_ref().unwrap(), "pwd");
        assert_eq!(url_components.authority.as_ref().unwrap()
                       .host, "somehost");
        assert_eq!(*url_components.authority.as_ref().unwrap()
                        .port.as_ref().unwrap() as u8, 80 as u8);
        assert_eq!(url_components.path, "/path");
        assert_eq!(url_components.query.as_ref().unwrap()
                       .get("param").unwrap(), "value");
        assert!(url_components.query.as_ref().unwrap()
                        .contains_key("anotherParam"));
        assert_eq!("", url_components.query.as_ref().unwrap()
                        .get("anotherParam").unwrap());

    }

    #[test]
    fn parse_simple_url_ftp() {
        let url = "ftp://ftp.is.co.za/rfc/rfc1808.txt";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "ftp");
        assert_eq!(url_components.authority.as_ref().unwrap().user_info, None);
        assert_eq!(url_components.authority.as_ref().unwrap()
                       .host, "ftp.is.co.za");
        assert_eq!(url_components.authority.as_ref().unwrap()
            .port, None);
        assert_eq!(url_components.path, "/rfc/rfc1808.txt");
        assert_eq!(url_components.query, None);
        assert_eq!(url_components.fragment, None);
    }

    #[test]
    fn parse_simple_url_ldap() {
        let url = "ldap://[2001:db8::7]/c=GB?objectClass?one";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "ldap");
        assert_eq!(url_components.authority.as_ref().unwrap().user_info, None);
        assert_eq!(url_components.authority.as_ref().unwrap()
                       .host, "[2001:db8::7]");
        assert_eq!(url_components.authority.as_ref().unwrap()
                       .port, None);
        assert_eq!(url_components.path, "/c=GB");
        assert!(url_components.query.unwrap().contains_key("objectClass?one"));
        assert_eq!(url_components.fragment, None);
    }

    #[test]
    fn parse_simple_url_news() {
        let url = "news:comp.infosystems.www.servers.unix";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "news");
        assert_eq!(url_components.authority, None);
        assert_eq!(url_components.path, "comp.infosystems.www.servers.unix");
        assert_eq!(url_components.query, None);
        assert_eq!(url_components.fragment, None);
    }

    #[test]
    fn parse_simple_url_tel() {
        let url = "tel:+1-816-555-1212";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "tel");
        assert_eq!(url_components.authority, None);
        assert_eq!(url_components.path, "+1-816-555-1212");
        assert_eq!(url_components.query, None);
        assert_eq!(url_components.fragment, None);
    }

    #[test]
    fn parse_simple_url_telnet() {
        let url = "telnet://192.0.2.16:80/";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "telnet");
        assert_eq!(url_components.authority.as_ref().unwrap().user_info, None);
        assert_eq!(url_components.authority.as_ref().unwrap().host, "192.0.2.16");
        assert_eq!(url_components.authority.as_ref().unwrap().port.unwrap(), 80);
        assert_eq!(url_components.path, "/");
        assert_eq!(url_components.query, None);
        assert_eq!(url_components.fragment, None);
    }

    #[test]
    fn parse_simple_url_mailto() {
        let url = "mailto:John.Doe@example.com";
        let url_components = parse_url(url).unwrap();


        assert_eq!(url_components.scheme, "mailto");
        assert_eq!(url_components.authority, None);
        assert_eq!(url_components.path, "John.Doe@example.com");
        assert_eq!(url_components.query, None);
        assert_eq!(url_components.fragment, None);
    }
}
