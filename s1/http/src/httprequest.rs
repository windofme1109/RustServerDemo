use std::collections::HashMap;



#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            Get => Method::Get,
            Post => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
}


#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String
}

impl From for HttpRequest {
    fn from(s: String) -> self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;

    }
}


#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized
}

impl From <&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get)
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1)
    }
}

