use std::collections::HashMap;
use std::io::Result;
use std::io::Write;

//  http 响应结构体如下：
/// 协议版本 状态码 状态码原因短语
/// key:value
/// key:value
/// ...
/// 
/// 
/// 响应体
/// 

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    // 字段类型是字符串切片，字符串切片是引用，所以需要标注其生命周期
    version: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    status_text: &'a str,
    status_code: &'a str,
    body: Option<String>
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
       Self {
           // into 方法完成类型转换，同时也转移了所有权
           version: "HTTP/1.1".into(),
            headers: None,
           status_code: "200".into(),
           status_text: "OK".into(),
           body: None
       } 
    }
}


impl<'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse<'a>) -> Self {

        let copied_response = response.clone();

        format!(
            "{} {} {}\r\nContent-Length:{}\r\n{}\r\n{}", 
            copied_response.version(),
            copied_response.status_code(),
            copied_response.status_text(),
            copied_response.body().len(),
            copied_response.headers(),
            copied_response.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code;
        }

        let mut h = HashMap::new();

        // 使用模式匹配处理响应头信息
        response.headers = match &headers {
            Some(_h) => {
                // 存在响应头信息，则直接使用响应头
                headers
            },
            None => {
                // 不存在响应头，新建一个
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        // 处理状态码对应的原因短语
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "404" => "Not Found".into(),
            "400" => "Bad Request".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        // 处理响应体
        // response.body = match &body {
        //     Some(_b) => {
        //         body
        //     },
        //     None => {
                
        //     }
        // };

        response.body = body;

        response
    }

    // write_stream 的类型是任何实现了 Write 这个 trait 的类型
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();

        let response_string = String::from(res);
        // write! 这个宏的作用是：向一个缓冲区里写格式化的数据
        // 这个宏接受一个'writer'，一个格式化字符串和一系列参数
        // 这些参数均会根据这个格式化的字符串被格式化，然后一并传给前面的 writer
        // 这个 writer 可以是任何带有 write_fmt 方法的值（可以理解为对象）
        // 一般来说这个值要么实现了 std::fmt::Write trait，要么实现了 std::io::Write trait
        // 这个宏返回write_fmt方法的返回值；一般是 std::fmt::Resul 或 std::io::Result
        let _ = write!(write_stream, "{}", response_string);

        Ok(())

    }

    // 实现 getter 方法
    pub fn version(&self) -> &str {
        self.version
    }

    pub fn status_code(&self) -> &str {
        self.status_code
    }

    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        // 获取 headers 的副本
        let headers = self.headers.clone().unwrap();

        // let mut header_string = "".into();
        let mut header_string = "".to_string();

        // 对 headers 进行遍历，将其转换为 字符串
        // 对 hashmap 进行遍历，需要调用 iter 方法将其转换为迭代器
        for (k, v) in headers.iter() {
            // push_str 接收的是 &str
            // format! 返回的是 String，因此需要进行转换
            header_string.push_str(format!("{}:{}\r\n", k, v).as_str());
        }

        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => {
                // String 转换为字符串切片
                b.as_str()
            },
            None => ""
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let mut h = HashMap::new();
        // h.insert("host", "localhost");
        h.insert("content-type", "plain/text");

        let res = HttpResponse::new("200", Some(h), Some("xxxx".into()));
        
        let test_res = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            body: Some("xxxx".into()),
            headers: {
                let mut h = HashMap::new();
                // h.insert("host", "localhost");
                h.insert("content-type", "plain/text");
                Some(h)
            }
        };

        assert_eq!(res, test_res);
    
    }

    #[test]
    fn test_response_struct_creation_404() {
        let mut h = HashMap::new();
        h.insert("host", "localhost");
        h.insert("content-type", "plain/text");

        let res = HttpResponse::new("404", Some(h), Some("abcd".into()));
        
        let test_res = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            body: Some("abcd".into()),
            headers: {
                let mut h = HashMap::new();
                h.insert("host", "localhost");
                h.insert("content-type", "plain/text");
                Some(h)
            }
        };

        assert_eq!(res, test_res);
    }


    #[test]
    fn test_response_struct_creation() {
      
        let res = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            body: Some("xxxx".into()),
            headers: {
                let mut h = HashMap::new();
                // h.insert("host", "localhost");
                h.insert("content-type", "plain/text");
                Some(h)
            }
        };

        let res_string: String = res.into();

        let test_res_string = format!(
            "HTTP/1.1 200 OK\r\ncontent-type:plain/text\r\n\r\nxxxx"
        );

        assert_eq!(res_string, test_res_string);
    }
}