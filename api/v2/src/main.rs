#![feature(lazy_cell, inline_const)]

// Best effort bais: when true is always true, when false may be true

use crate::query::{Entry, Query};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use query::QueryResponse;
use std::collections::HashMap;

pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");
const GOOGLE_PROJECT_ID: &str = "hidden-sunlight-414619";

// let key - "$(gcloud auth print-access-token)"),

mod google;
mod query;
mod search;

#[derive(Debug)]
pub struct State {
    pub data: HashMap<String, HashMap<String, usize>>,
    // pub key: String,
}

/// Search based on exact matches
#[get("/v2/get/{name}")]
async fn get(path: web::Path<Query>, state: web::Data<State>) -> impl Responder {
    let query = path.as_ref();

    log::info!(
        target: "/v2/get",
        "Request: {:?}", query
    );

    let Query::Names(names) = query else {
        return HttpResponse::from(
            QueryResponse::new(query.clone()).error("Error: query should be a Name"),
        );
    };

    let response = make_matches(state.as_ref(), names).await;

    HttpResponse::from(response)
}

async fn make_matches(state: &State, names: &[String]) -> QueryResponse {
    let mut response = QueryResponse::new(names.to_vec());

    for name in names {
        if let Some(value) = state.data.get(name) {
            response.push(Entry::new(name.clone(), 1.0, value.clone()))
        }
    }
    QueryResponse::new(String::new())
}

/// Search based using fuzzy finder
#[get("/v2/find/{name}")]
async fn find(path: web::Path<Query>, state: web::Data<State>) -> impl Responder {
    let query = path.as_ref();

    let Query::Names(names) = query else {
        let response = QueryResponse::new(query.clone()).error(String::from("yoy fuck"));
        return HttpResponse::from(response);
    };

    let mut response = QueryResponse::new(names.clone());
    for name in names {
        for (entry, resp) in state.data.iter() {
            let sim = strsim::jaro(&name, entry.to_lowercase().as_str());
            if sim > 0.80 {
                response.push(Entry::new(entry.clone(), sim, resp.clone()))
            }
        }
    }
    HttpResponse::from(response)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[allow(unused_unsafe)] // will be needed in future versions
    unsafe {
        std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,ecosnap")
    };

    logger::init();

    HttpServer::new(|| {
        let data = json::from_str(include_str!("../../data.json")).unwrap();

        App::new()
            .app_data(web::Data::new(State { data }))
            .service(search::search)
            .service(get)
            .service(find)
    })
    .bind(("127.0.0.1", 6699))?
    .run()
    .await
}

