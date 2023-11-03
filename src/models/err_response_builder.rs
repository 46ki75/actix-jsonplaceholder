use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ErrResponseBuilder {
    id: String,
    code: u16,
    status: String,
    source: Source,
    title: String,
    detail: Option<String>,
    timestamp: String,
}

#[derive(Serialize, Deserialize)]
struct Source {
    pointer: Option<String>,
    parameter: Option<String>,
}

#[allow(dead_code)]
impl ErrResponseBuilder {
    pub fn new(code: u16) -> Self {
        let status = match code {
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            422 => "Unprocessable Entity",
            423 => "Locked",
            424 => "Failed Dependency",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network Authentication Required",
            _ => "Unknown Error",
        };

        Self {
            id: Uuid::new_v4().to_string(),
            code,
            status: String::from(status),
            source: Source {
                pointer: None,
                parameter: None,
            },
            title: String::from(status),
            detail: None,
            timestamp: String::from(""),
        }
    }

    pub fn detail<S: AsRef<str>>(mut self, detail: S) -> Self {
        self.detail = Some(detail.as_ref().to_string());
        self
    }

    pub fn pointer<S: AsRef<str>>(mut self, pointer: S) -> Self {
        self.source.pointer = Some(pointer.as_ref().to_string());
        self
    }

    pub fn parameter<S: AsRef<str>>(mut self, parameter: S) -> Self {
        self.source.parameter = Some(parameter.as_ref().to_string());
        self
    }

    pub fn build(mut self) -> Self {
        self.timestamp = Utc::now().with_timezone(&Tokyo).to_rfc3339();
        self
    }
}
