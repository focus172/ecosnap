use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    names: Vec<String>,
    image: bool,
}

impl From<String> for Query {
    fn from(value: String) -> Self {
        Self {
            names: vec![value],
            image: false,
        }
    }
}

impl From<Vec<String>> for Query {
    fn from(value: Vec<String>) -> Self {
        Self {
            names: value,
            image: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    name: String,
    matchs: f64,
    scores: HashMap<String, usize>,
}

impl Entry {
    pub fn new(name: String, matchs: f64, scores: HashMap<String, usize>) -> Self {
        Self {
            name,
            matchs,
            scores,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    data: Result<Vec<Entry>, String>,
    query: Query,
}

impl QueryResponse {
    pub fn new(query: impl Into<Query>) -> Self {
        Self {
            data: Ok(Vec::default()),
            query: query.into(),
        }
    }

    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.data = Err(msg.into());
        self
    }

    pub fn push(&mut self, value: Entry) {
        let Ok(r) = &mut self.data else {
            log::warn!("Attempted to push data to `Err` response.");
            return;
        };
        r.push(value)
    }
}

impl From<QueryResponse> for HttpResponse {
    fn from(mut value: QueryResponse) -> Self {
        if value.data.as_ref().is_ok_and(|v| v.is_empty()) {
            value.data = Err(String::from("Could not find any matches."))
        }

        HttpResponse::Ok()
            .content_type("application/json")
            .json(value)
    }
}
