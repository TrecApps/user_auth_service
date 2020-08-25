

pub enum HttpResponseCodeTypes
{
    Info100, // Continue
    Info101, // Switching Protocol
    Info102, // Processing
    Info103, // Early Hints

    Success200, // Ok
    Success201, // Created
    Success202, // Accepted
    Success203, // Non-Authoritative Information
    Success204, // No Content
    Success205, // Reset Content
    Success206, // Partial Content
    Success207, // Multi-Status
    Success208, // Already Reported
    Success226, // IM Used 

    Redirect300, // Multiple Choice
    Redirect301, // Moved Permanently
    Redirect302, // Found
    Redirect303, // See Other
    Redirect304, // Not Modified
    Redirect305, // Use Proxy (DO NOT USE)
    Redirect306, // unused (NO LONGER USED)
    Redirect307, // Temporary Redirect
    Redirect308, // Permanent Redirect

    ClientErr400, // Bad Request
    ClientErr401, // Unauthorized
    ClientErr402, // Payment Required
    ClientErr403, // Forbidden
    ClientErr404, // Not Found
    ClientErr405, // Method Not Allowed
    ClientErr406, // Not Acceptable
    ClientErr407, // Proxy Authentication Required
    ClientErr408, // Request Timeout
    ClientErr409, // Conflict
    ClientErr410, // Gone
    ClientErr411, // Length Required
    ClientErr412, // Precondition Failed
    ClientErr413, // Payload Too Large
    ClientErr414, // URI Too Long
    ClientErr415, // Unsupported Media Type
    ClientErr416, // Range Not Satisfiable
    ClientErr417, // Expectation Failed
    ClientErr418, // I'm a teapot
    ClientErr421, // Misdirected Request
    ClientErr422, // Unprocessable Entity 
    ClientErr423, // Locked 
    ClientErr424, // Failed Dependency
    ClientErr425, // Too Early
    ClientErr426, // Upgrade Required
    ClientErr428, // Precondition Required
    ClientErr429, // Too Many Requests
    ClientErr431, // Request Header Fields Too Large
    ClientErr451, // Unavailable For Legal Reasons

    ServerErr500, // Internal Server Error
    ServerErr501, // Not Implemented
    ServerErr502, // Bad Gateway
    ServerErr503, // Service Unavailable
    ServerErr504, // Gateway Timeout
    ServerErr505, // HTTP Version Not Supported
    ServerErr506, // Variant Also Negotiates
    ServerErr507, // Insufficient Storage 
    ServerErr508, // Loop Detected 
    ServerErr510, // Not Extended
    ServerErr511 // Network Authentication Required
    
}

pub struct HttpResponseCode
{
    status: u16,
    status_str: String
}

impl HttpResponseCode
{
    pub fn new(code_type: HttpResponseCodeTypes) -> HttpResponseCode
    {
        match code_type
        {
            HttpResponseCodeTypes::Info100 => HttpResponseCode{ status: 100, status_str: String::from("Continue")},  
            HttpResponseCodeTypes::Info101 => HttpResponseCode{ status: 101, status_str: String::from("Switching Protocol")},  
            HttpResponseCodeTypes::Info102 => HttpResponseCode{ status: 102, status_str: String::from("Processing")},  
            HttpResponseCodeTypes::Info103 => HttpResponseCode{ status: 103, status_str: String::from("Early Hints")},  
        
            HttpResponseCodeTypes::Success200 => HttpResponseCode{ status: 200, status_str: String::from("Ok")},  
            HttpResponseCodeTypes::Success201 => HttpResponseCode{ status: 201, status_str: String::from("Created")},  
            HttpResponseCodeTypes::Success202 => HttpResponseCode{ status: 202, status_str: String::from("Accepted")},  
            HttpResponseCodeTypes::Success203 => HttpResponseCode{ status: 203, status_str: String::from("Non-Authoritative Information")},  
            HttpResponseCodeTypes::Success204 => HttpResponseCode{ status: 204, status_str: String::from("No Content")},  
            HttpResponseCodeTypes::Success205 => HttpResponseCode{ status: 205, status_str: String::from("Reset Content")},  
            HttpResponseCodeTypes::Success206 => HttpResponseCode{ status: 206, status_str: String::from("Partial Content")},  
            HttpResponseCodeTypes::Success207 => HttpResponseCode{ status: 207, status_str: String::from("Multi-Status")},  
            HttpResponseCodeTypes::Success208 => HttpResponseCode{ status: 208, status_str: String::from("Already Reported")},  
            HttpResponseCodeTypes::Success226 => HttpResponseCode{ status: 226, status_str: String::from("IM Used")},   
        
            HttpResponseCodeTypes::Redirect300 => HttpResponseCode{ status: 300, status_str: String::from("Multiple Choice")},  
            HttpResponseCodeTypes::Redirect301 => HttpResponseCode{ status: 301, status_str: String::from("Moved Permanently")},  
            HttpResponseCodeTypes::Redirect302 => HttpResponseCode{ status: 302, status_str: String::from("Found")},  
            HttpResponseCodeTypes::Redirect303 => HttpResponseCode{ status: 303, status_str: String::from("See Other")},  
            HttpResponseCodeTypes::Redirect304 => HttpResponseCode{ status: 304, status_str: String::from("Not Modified")},  
            HttpResponseCodeTypes::Redirect305 => HttpResponseCode{ status: 305, status_str: String::from("Use Proxy")}, //  (DO NOT USE)
            HttpResponseCodeTypes::Redirect306 => HttpResponseCode{ status: 306, status_str: String::from("unused")}, //  (NO LONGER USED)
            HttpResponseCodeTypes::Redirect307 => HttpResponseCode{ status: 307, status_str: String::from("Temporary Redirect")},  
            HttpResponseCodeTypes::Redirect308 => HttpResponseCode{ status: 308, status_str: String::from("Permanent Redirect")},  
        
            HttpResponseCodeTypes::ClientErr400 => HttpResponseCode{ status: 400, status_str: String::from("Bad Request")},  
            HttpResponseCodeTypes::ClientErr401 => HttpResponseCode{ status: 401, status_str: String::from("Unauthorized")},  
            HttpResponseCodeTypes::ClientErr402 => HttpResponseCode{ status: 402, status_str: String::from("Payment Required")},  
            HttpResponseCodeTypes::ClientErr403 => HttpResponseCode{ status: 403, status_str: String::from("Forbidden")},  
            HttpResponseCodeTypes::ClientErr404 => HttpResponseCode{ status: 404, status_str: String::from("Not Found")},  
            HttpResponseCodeTypes::ClientErr405 => HttpResponseCode{ status: 405, status_str: String::from("Method Not Allowed")},  
            HttpResponseCodeTypes::ClientErr406 => HttpResponseCode{ status: 406, status_str: String::from("Not Acceptable")},  
            HttpResponseCodeTypes::ClientErr407 => HttpResponseCode{ status: 407, status_str: String::from("Proxy Authentication Required")},  
            HttpResponseCodeTypes::ClientErr408 => HttpResponseCode{ status: 408, status_str: String::from("Request Timeout")},  
            HttpResponseCodeTypes::ClientErr409 => HttpResponseCode{ status: 409, status_str: String::from("Conflict")},  
            HttpResponseCodeTypes::ClientErr410 => HttpResponseCode{ status: 410, status_str: String::from("Gone")},  
            HttpResponseCodeTypes::ClientErr411 => HttpResponseCode{ status: 411, status_str: String::from("Length Required")},  
            HttpResponseCodeTypes::ClientErr412 => HttpResponseCode{ status: 412, status_str: String::from("Precondition Failed")},  
            HttpResponseCodeTypes::ClientErr413 => HttpResponseCode{ status: 413, status_str: String::from("Payload Too Large")},  
            HttpResponseCodeTypes::ClientErr414 => HttpResponseCode{ status: 414, status_str: String::from("URI Too Long")},  
            HttpResponseCodeTypes::ClientErr415 => HttpResponseCode{ status: 415, status_str: String::from("Unsupported Media Type")},  
            HttpResponseCodeTypes::ClientErr416 => HttpResponseCode{ status: 416, status_str: String::from("Range Not Satisfiable")},  
            HttpResponseCodeTypes::ClientErr417 => HttpResponseCode{ status: 417, status_str: String::from("Expectation Failed")},  
            HttpResponseCodeTypes::ClientErr418 => HttpResponseCode{ status: 418, status_str: String::from("I'm a teapot")},  
            HttpResponseCodeTypes::ClientErr421 => HttpResponseCode{ status: 421, status_str: String::from("Misdirected Request")},  
            HttpResponseCodeTypes::ClientErr422 => HttpResponseCode{ status: 422, status_str: String::from("Unprocessable Entity")},   
            HttpResponseCodeTypes::ClientErr423 => HttpResponseCode{ status: 423, status_str: String::from("Locked")},   
            HttpResponseCodeTypes::ClientErr424 => HttpResponseCode{ status: 424, status_str: String::from("Failed Dependency")},  
            HttpResponseCodeTypes::ClientErr425 => HttpResponseCode{ status: 425, status_str: String::from("Too Early")},  
            HttpResponseCodeTypes::ClientErr426 => HttpResponseCode{ status: 426, status_str: String::from("Upgrade Required")},  
            HttpResponseCodeTypes::ClientErr428 => HttpResponseCode{ status: 428, status_str: String::from("Precondition Required")},  
            HttpResponseCodeTypes::ClientErr429 => HttpResponseCode{ status: 429, status_str: String::from("Too Many Requests")},  
            HttpResponseCodeTypes::ClientErr431 => HttpResponseCode{ status: 431, status_str: String::from("Request Header Fields Too Large")},  
            HttpResponseCodeTypes::ClientErr451 => HttpResponseCode{ status: 451, status_str: String::from("Unavailable For Legal Reasons")},  
        
            HttpResponseCodeTypes::ServerErr500 => HttpResponseCode{ status: 500, status_str: String::from("Internal Server Error")},  
            HttpResponseCodeTypes::ServerErr501 => HttpResponseCode{ status: 501, status_str: String::from("Not Implemented")},  
            HttpResponseCodeTypes::ServerErr502 => HttpResponseCode{ status: 502, status_str: String::from("Bad Gateway")},  
            HttpResponseCodeTypes::ServerErr503 => HttpResponseCode{ status: 503, status_str: String::from("Service Unavailable")},  
            HttpResponseCodeTypes::ServerErr504 => HttpResponseCode{ status: 504, status_str: String::from("Gateway Timeout")},  
            HttpResponseCodeTypes:: ServerErr505 => HttpResponseCode{ status: 505, status_str: String::from("HTTP Version Not Supported")},  
            HttpResponseCodeTypes::ServerErr506 => HttpResponseCode{ status: 506, status_str: String::from("Variant Also Negotiates")},  
            HttpResponseCodeTypes::ServerErr507 => HttpResponseCode{ status: 507, status_str: String::from("Insufficient Storage")},   
            HttpResponseCodeTypes::ServerErr508 => HttpResponseCode{ status: 508, status_str: String::from("Loop Detected")},   
            HttpResponseCodeTypes::ServerErr510 => HttpResponseCode{ status: 510, status_str: String::from("Not Extended")},  
            HttpResponseCodeTypes::ServerErr511 => HttpResponseCode{ status: 511, status_str: String::from("Network Authentication Required")}  
        }
    }

    pub fn get_status(&self) -> u16
    {
        self.status
    }

    pub fn get_status_str(&self) -> String
    {
        self.status_str.clone()
    }
}