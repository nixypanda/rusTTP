#[derive(Debug, PartialEq, Eq)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
}

#[derive(Debug)]
pub(crate) struct Response {
    status_code: StatusCode,
    headers: Vec<String>,
    content: String,
}

impl Response {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        Vec::from(self.to_string().as_bytes())
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let protocol = "HTTP/1.1";
        let status_code = match self.status_code {
            StatusCode::Ok => "200 OK",
            StatusCode::NotFound => "404 Not Found",
        };
        let header_string = self.headers.join("\r\n");

        let content = format!(
            "{protocol} {status_code}\r\n{headers}\r\n\r\n{content}",
            protocol = protocol,
            status_code = status_code,
            headers = header_string,
            content = self.content
        );
        write!(f, "{}", content)
    }
}

#[derive(Debug)]
pub(crate) enum StatusCode {
    Ok,
    NotFound,
}

pub(crate) struct ResponseBuilder {
    status_code: StatusCode,
    headers: Vec<String>,
    content: String,
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            status_code: StatusCode::Ok,
            headers: vec![],
            content: "".to_string(),
        }
    }

    pub fn status_code(mut self, status_code: StatusCode) -> ResponseBuilder {
        self.status_code = status_code;
        self
    }

    pub fn build(self) -> Response {
        Response {
            status_code: self.status_code,
            headers: self.headers,
            content: self.content,
        }
    }
}
