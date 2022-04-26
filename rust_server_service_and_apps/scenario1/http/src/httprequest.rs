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


/*
- HttpResponse에서는 &str을 사용하고 HttpReqeust에서는 String을 사용하는 이유 -
웹 서버가 HTTP 응답을 다시 보낼 때 다시 보내야 하는 헤더를 정확히 알고 있으므로 정적 문자열(&str)을 사용할 수 있다.
들어오는 요청을 처리할 때 웹 서버는 헤더에 대해 미리 알지 못하므로 동적으로 메모리를 할당해야 하므로 String을 사용한다.
*/

pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // 각 줄을 읽음
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
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    // into()를 사용하면 받는쪽의 명시된 타입의 from이 호출된다... 엄청난 magic
    (method.into(), Resource::Path(resource.to_string()), version.into())
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    // 헤더의 키 부분 추출
    if let Some(k) = header_items.next() {
        key = k.to_string()
    }

    // 헤더의 값 부분 추출
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }


    /*
    // if let과 if와의 차이점은 패턴 매칭이냐, boolean값이냐의 여부 차이이다.
    // 위의 if let 코드는 다음과 같다
    match header_items.next() {
        Some(x) => {
            key = k.to_string();
        }
        None => {}
    }
     */


    (key, value)
}