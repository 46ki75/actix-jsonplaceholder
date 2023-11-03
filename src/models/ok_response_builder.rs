use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct OkResponseBuilder<T: Serialize> {
    meta: Meta,
    data: Option<Vec<T>>,
    links: Links,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    id: String,
    total_pages: Option<u32>,
    timestamp: String,
}

#[derive(Deserialize, Serialize)]
struct Links {
    #[serde(rename = "self")]
    self_link: Option<String>,

    #[serde(rename = "first")]
    first_link: Option<String>,

    #[serde(rename = "prev")]
    prev_link: Option<String>,

    #[serde(rename = "next")]
    next_link: Option<String>,

    #[serde(rename = "last")]
    last_link: Option<String>,
}

#[allow(dead_code)]
impl<T: Serialize> OkResponseBuilder<T> {
    pub fn new() -> Self {
        Self {
            meta: Meta {
                id: Uuid::new_v4().to_string(),
                total_pages: None,
                timestamp: Utc::now().with_timezone(&Tokyo).to_rfc3339(),
            },
            data: None,
            links: Links {
                self_link: None,
                first_link: None,
                prev_link: None,
                next_link: None,
                last_link: None,
            },
        }
    }

    pub fn total_pages(mut self, total_pages: u32) -> Self {
        self.meta.total_pages = Some(total_pages);
        self
    }

    pub fn data(mut self, data: Vec<T>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn push_data(mut self, data: T) -> Self {
        match self.data {
            Some(ref mut vec) => vec.push(data),
            None => self.data = Some(vec![data]),
        };
        self
    }

    pub fn self_link<S: AsRef<str>>(mut self, self_link: S) -> Self {
        self.links.self_link = Some(self_link.as_ref().to_string());
        self
    }

    pub fn first_link<S: AsRef<str>>(mut self, first_link: S) -> Self {
        self.links.first_link = Some(first_link.as_ref().to_string());
        self
    }

    pub fn prev_link<S: AsRef<str>>(mut self, prev_link: S) -> Self {
        self.links.prev_link = Some(prev_link.as_ref().to_string());
        self
    }

    pub fn next_link<S: AsRef<str>>(mut self, next_link: S) -> Self {
        self.links.next_link = Some(next_link.as_ref().to_string());
        self
    }

    pub fn last_link<S: AsRef<str>>(mut self, last_link: S) -> Self {
        self.links.last_link = Some(last_link.as_ref().to_string());
        self
    }

    pub fn build(mut self) -> Self {
        self.data.get_or_insert_with(Vec::new);
        self.meta.timestamp = Utc::now().with_timezone(&Tokyo).to_rfc3339();
        self
    }
}
