use super::uri::Uri;
use std::collections::HashMap;

pub enum Method{
    Post,
    Get,
    Put,
    Delete
}
pub struct Request {
    pub version: String,
    pub method: Method,
    pub uri: Uri,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>    
}
impl Request {
    pub fn from(str: String) -> Result<Request, i16> {
        let mut lines: Vec<&str> = str.as_str().split("\r\n").collect();
        let request_line: Vec<&str> = lines.remove(0).split(" ").collect();
        let mut method;
        match request_line[0] {
            "POST" => {
                method=Method::Post;
            },
            "GET" => {
                method=Method::Get;
            },
            "PUT" => {
                method=Method::Put;
            },
            "DELETE" => {
                method=Method::Delete;
            },
            &_ => {
                return Err(501);
            }
        };
        let uri = Uri::from(request_line[1].trim().to_string());
        let version = request_line[2].to_string();
        let mut headers = HashMap::new();
        let mut l = String::from(lines.remove(0));
        while l != "" {
            let kv: Vec<&str> = l.split(":").collect();
            headers.insert(
                kv[0].trim().to_string(),
                kv[1].trim().to_string()
            );
            l = lines.remove(0).to_string();
        }
        let body=lines.join("\r\n").as_bytes().to_vec();
        Ok(Request {
            version: version,
            method: method,
            uri: uri,
            headers: headers,
            body: body,
        })
    }
}

