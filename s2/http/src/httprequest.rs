use std::{collections::HashMap};


/**
 * 定义 Http Request 结构
 * 
 */

//  定义请求方法
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized
}

/**
 * 将传进来的字符串切片转换为 Method 枚举
 */
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
           get => Method::Get,
           post => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

// 定义 http 协议版本
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "http/1.1" => Version::V1_1,
            "http/2" => Version::V2_0,
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub resource: Resource,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        // 初始化请求方法、协议版本、请求资源（路径）
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());

        let mut parsed_headers: HashMap<String, String> = HashMap::new();

        let mut parsed_msg_body = "".to_string();

        for line in req.lines() {
            
            if line.to_lowercase().contains("http") {
                // 解析首行
                let (method, version, resource) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                // 解析 header
                let (key, value) = process_header_line(line);
                // 将获取到的请求头键值对插入 headers 这个 hashMap 中
                parsed_headers.insert(key, value);
                
            } else {
                parsed_msg_body = line.to_string();
            }
        }


        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            headers: parsed_headers,
            resource: parsed_resource,
            msg_body: parsed_msg_body,
        }

    }
}

// 对 http 请求的首行进行解析
fn process_req_line(s: &str) -> (Method, Version, Resource) {
    // http 请求的首行格式：请求方法 请求路径 协议版本\r\n
    let mut content = s.split_ascii_whitespace();
    let method = content.next().unwrap();
    let path = content.next().unwrap();
    let version = content.next().unwrap();

    // (
    //     Method::from(method),
    //     Version::from(version),
    //     Resource::Path(path.to_string()),

    // )

    (
        method.into(),
        version.into(),
        Resource::Path(path.to_string()),
    )
}


fn process_header_line(s: &str) -> (String, String) {
    let mut header = s.split(":");
    let mut key = "".to_string();
    let mut val = "".to_string();

    if let Some(k) = header.next() {
        key = k.to_string();
    }

    if let Some(v) = header.next() {
        val = v.to_string();
    }
   
    (
        key,
        val
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();

        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "http/1.1".into();

        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {

        let request_str = String::from("Get /api/name http/1.1\r\ncontent-type:plain/text\r\n\r\n");

        let parsed_request = HttpRequest::from(request_str);

        let mut headers = HashMap::new();
        headers.insert("content-type".into(), "plain/text".into());

        let request = HttpRequest {
            method: Method::Get,
            version: Version::V1_1,
            resource: Resource::Path("/api/name".to_string()),
            headers,
            msg_body: "".to_string()
        };


        assert_eq!(request, parsed_request);
    }
}