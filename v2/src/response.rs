use std::collections::HashMap;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::Query;

#[derive(Debug, Serialize, Deserialize)]
pub struct Thing {
    name: String,
    matchs: f64,
    scores: HashMap<String, usize>,
}

impl Thing {
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
    data: Result<Vec<Thing>, String>,
    query: Query,
}

impl QueryResponse {
    pub fn query(query: impl Into<Query>) -> Self {
        Self {
            data: Ok(Vec::default()),
            query: query.into(),
        }
    }

    pub fn error(mut self, msg: String) -> Self {
        self.data = Err(msg);
        self
    }

    pub fn push(&mut self, value: Thing) {
        let Ok(r) = &mut self.data else { return };
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
