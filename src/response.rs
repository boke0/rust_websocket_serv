use std::collections::HashMap;

pub struct Response {
    pub status: i16,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}
impl Response {
    pub fn new(code: i16, version: String) -> Response{
        Response {
            status: code,
            version: version,
            headers: HashMap::new(),
            body: Vec::new()
        }
    }
    pub fn to_vec(&mut self) -> Vec<u8>{
        let mut bytes = Vec::new();
        let mut t=format!(
                "HTTP/{} {} {}\r\n",
                self.version,
                self.status,
                self.get_reason()
            ).as_bytes()
            .to_vec();
        bytes.append(&mut t);
        for (k,v) in &self.headers {
            let mut t=format!("{}: {}\r\n",k,v)
                    .as_bytes()
                    .to_vec();
            bytes.append(&mut t);   
        }
        let mut t="\r\n".as_bytes().to_vec();
        bytes.append(&mut t);
        bytes.append(&mut self.body);
        bytes
    }
    pub fn get_reason(&self) -> String {
        match self.status {
            100 => "Continue".to_string(),
            101 => "Switching Protocol".to_string(),
            102 => "Processing".to_string(),
            103 => "Early Hints".to_string(),
            200 => "OK".to_string(),
            201 => "Created".to_string(),
            202 => "Accepted".to_string(),
            203 => "Non-Authoritative Information".to_string(),
            204 => "No Content".to_string(),
            205 => "Reset Content".to_string(),
            206 => "Partial Content".to_string(),
            207 => "Multi-Status".to_string(),
            208 => "Already Reported".to_string(),
            300 => "Multiple Choices".to_string(),
            301 => "Moved Permanently".to_string(),
            302 => "Found".to_string(),
            303 => "See Other".to_string(),
            304 => "Not Modified".to_string(),
            305 => "Use Proxy".to_string(),
            306 => "Switched Proxy".to_string(),
            307 => "Temporary Redirect".to_string(),
            308 => "Permanent Redirect".to_string(),
            400 => "Bad Request".to_string(),
            401 => "Unauthorized".to_string(),
            402 => "Payment Required".to_string(),
            403 => "Forbidden".to_string(),
            404 => "Not Found".to_string(),
            405 => "Method Not Allowed".to_string(),
            406 => "Not Acceptable".to_string(),
            407 => "Proxy Authentication Required".to_string(),
            408 => "Request Timeout".to_string(),
            409 => "Conflict".to_string(),
            410 => "Gone".to_string(),
            411 => "Length Required".to_string(),
            412 => "Precondition Failed".to_string(),
            413 => "Payload Too Large".to_string(),
            414 => "URI Too Long".to_string(),
            415 => "Unsupported Media Type".to_string(),
            416 => "Range Not Satisfiable".to_string(),
            417 => "Expectation Failed".to_string(),
            418 => "I'm a teapot".to_string(),
            421 => "Misdirected Request".to_string(),
            422 => "Unprocessable Entity".to_string(),
            423 => "Locked".to_string(),
            424 => "Failed Dependency".to_string(),
            425 => "Too Early".to_string(),
            426 => "Upgrade Required".to_string(),
            428 => "Precondition Required".to_string(),
            429 => "Too Many Requests".to_string(),
            431 => "Request Header Fields Too Large".to_string(),
            451 => "Unavailable For Legal Reasons".to_string(),
            500 => "Internal Server Error".to_string(),
            501 => "Not Implemented".to_string(),
            502 => "Bad Gateway".to_string(),
            503 => "Service Unavailable".to_string(),
            504 => "Gateway Timeout".to_string(),
            505 => "HTTP Version Not Supported".to_string(),
            506 => "Variant Also Negotiates".to_string(),
            507 => "Insufficient Storage".to_string(),
            508 => "Loop Detected".to_string(),
            509 => "Bandwidth Limit Exceeded".to_string(),
            510 => "Not Extended".to_string(),
            511 => "Network Authentication Required".to_string(),
            _ => "あほーどりがないてる".to_string()
        }
    }
    pub fn get_mime(ext: String) -> String {
        match ext.as_str() {
            "txt" => "text/plain",
            "htm" => "text/html",
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "json" => "application/json",
            "xml" => "application/xml",
            "swf" => "application/x-shockwave-flash",
            "flv" => "video/x-flv",
            "png" => "image/png",
            "jpe" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "jpg" => "image/jpeg",
            "gif" => "image/gif",
            "bmp" => "image/bmp",
            "ico" => "image/vnd.microsoft.icon",
            "tiff" => "image/tiff",
            "tif" => "image/tiff",
            "svg" => "image/svg+xml",
            "svgz" => "image/svg+xml",
            "zip" => "application/zip",
            "rar" => "application/x-rar-compressed",
            "exe" => "application/x-msdownload",
            "msi" => "application/x-msdownload",
            "cab" => "application/vnd.ms-cab-compressed",
            "mp3" => "audio/mpeg",
            "qt" => "video/quicktime",
            "mov" => "video/quicktime",
            "pdf" => "application/pdf",
            "psd" => "image/vnd.adobe.photoshop",
            "ai" => "application/postscript",
            "eps" => "application/postscript",
            "ps" => "application/postscript",
            "doc" => "application/msword",
            "rtf" => "application/rtf",
            "xls" => "application/vnd.ms-excel",
            "ppt" => "application/vnd.ms-powerpoint",
            "odt" => "application/vnd.oasis.opendocument.text",
            "ods" => "application/vnd.oasis.opendocument.spreadsheet",
            _ => "text/plain"
        }.to_string()
    }
}
