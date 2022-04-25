use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get, Post, Uninitialized
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
}

pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from( req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // 각 줄을 일긍ㅁ
        for line in req.lines() {
            // If the line read is request line, call function process_req_line()
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
                // If the line read is header line, call function process_header_line()
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
                //  If it is blank line, do nothing
            } else if line.len() == 0 {
                // If none of these, treat it as message body
            } else {
                parsed_msg_body = line;
            }
        }
        // Parse the incoming HTTP request into HttpRequest struct
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }


}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // split_whitespace: 공백을 기준으로 Option(Some(), Nome) Iterator을 가진 SplitWhitespace 구조체를 반환
    let mut words = s.split_whitespace();
    let mut method = words.next().unwrap();
    let mut resource = words.next().unwrap();
    let mut version = words.next().unwrap();

    // into()를 사용하면 받는쪽의 명시된 타입의 from이 호출된다... 엄청난 magic
    (method.into(), Result::Path(resource.to_string()), version.into())
}