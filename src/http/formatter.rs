pub struct HttpFormatter<'a> {
    request_protocol: &'a str,
}

impl<'a> HttpFormatter<'a> {
    pub fn new(request_protocol: &'a str) -> Self {
        HttpFormatter { request_protocol }
    }

    pub fn format_response(self, contents: &String, code: u16, status: &'a str) -> String {
        let status_line = format!("{} {code} {status}", self.request_protocol);
        let content_length = contents.len();
        format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}")
    }
}
