use std::collections::HashMap;
use crate::controller::http_response_code;
use std::net::TcpStream;

use core::ops::Add;
use std::io::Write;


pub struct HttpResponse
{
    http_version: String,
    response_code: http_response_code::HttpResponseCode,
    headers: HashMap<String, String>,
    body: String
}

pub fn get_client_respone_using_text(body: String) -> HttpResponse
{
    let mut res = HttpResponse::new(http_response_code::HttpResponseCode::new(http_response_code::HttpResponseCodeTypes::ClientErr400));

    let key = String::from("Content-Type");
    let value = String::from("text/plain; charset=UTF-8");
    res.add_header(&key, &value);
    res.set_body(body);

    res
}

impl HttpResponse
{
    pub fn new(response_code: http_response_code::HttpResponseCode) -> HttpResponse
    {
        HttpResponse {
            http_version : String::from("HTTP/1.1"),
            response_code,
            headers : HashMap::<String, String>::new(),
            body: String::from("")
        }
    }

    pub fn add_header(&mut self, key: &String, value: &String) -> bool
    {
        if value.find("\r").is_some() || value.find("\n").is_some()
        {
            false
        }
        else
        {
            self.headers.insert(key.to_string(), value.to_string());

            true
        }
    }

    pub fn set_body(&mut self, b: String)
    {
        self.body = b;

        let (key, value) = (String::from("Content-Length"), format!("{}", self.body.len()));

        self.add_header(&key, &value);
    }

    pub fn write_response(&self, mut stream: &TcpStream)
    {
        let mut header_str = String::from("");

        for header in self.headers.iter()
        {
            let header_line = format!("{}: {}\r\n", header.0, header.1);
            header_str = header_str.add(header_line.as_str());
        }

        let response_str = format!("{} {} {}\r\n{}\r\n{}", self.http_version, self.response_code.get_status(), self.response_code.get_status_str(), header_str, self.body);

        stream.write(response_str.as_bytes());
    }
}