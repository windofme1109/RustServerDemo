use std::collections::HashMap;
use std::fmt::format;
use std::io::{Write, Result};


#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>
    ) -> HttpResponse<'a> {
        let mut repsonse: HttpResponse<'a> = HttpResponse::default();
        
        // let b = body.unwrap();
        // let content_length = b.clone().len().to_string().as_str();
        
        let body_string = body.unwrap();
      
        if status_code != "200" {
            repsonse.status_code = status_code.into();
        }

        repsonse.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");           
                h.insert("Content-Length", );           
                Some(h)
            }
        };


        repsonse.status_text = match repsonse.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        repsonse.body = body;

        return repsonse;

    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }  


    // 实现一些 getter 方法
    pub fn version(&self) -> &str {
        return self.version
    }  
    pub fn status_code(&self) -> &str {
        return self.status_code
    }  
    pub fn status_text(&self) -> &str {
        return self.status_text
    }  
    pub fn headers(&self) -> String {
        
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();

        let mut header_string = "".into();
        
        for (k, v) in map.iter() {
            header_string = format!("{}{}: {}\r\n", header_string, k, v);
        }

        return header_string;

    }  

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => ""
        }
    }

}


impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();

        format!(
            "{} {} {}\r\n{}Content-Lenght: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into())
        };

        assert_eq!(response_actual, response_expected)
    
    }


    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into())
        };

        assert_eq!(response_actual, response_expected)
    
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into())
        };

        let http_string: String = response_expected.into();
        
        let actual_string: String = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Lenght: 4\r\n\r\nxxxx".into();

        assert_eq!(actual_string, http_string);

    }
}