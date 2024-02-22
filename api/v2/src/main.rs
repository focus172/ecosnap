#![feature(lazy_cell, inline_const)]

// Best effort bais: when true is always true, when false may be true

use crate::query::Entry;
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
async fn get(path: web::Path<String>, state: web::Data<State>) -> impl Responder {
    let name = path.as_ref().clone();

    log::info!(
        target: "/v2/get",
        "Request: {:?}", name
    );

    let response = make_matches(state.as_ref(), &[name]).await;

    HttpResponse::from(response)
}

async fn make_matches(state: &State, names: &[String]) -> QueryResponse {
    let mut response = QueryResponse::new(names.to_vec());

    for name in names {
        if let Some(value) = state.data.get(name) {
            response.push(Entry::new(name.clone(), 1.0, value.clone()))
        }
    }
    response
}

/// Search based using fuzzy finder
#[get("/v2/find/{name}")]
async fn find(path: web::Path<String>, state: web::Data<State>) -> impl Responder {
    let name = path.as_ref();

    let mut response = QueryResponse::new(name.clone());
    for (entry, resp) in state.data.iter() {
        let sim = strsim::jaro(name.to_lowercase().as_str(), entry.to_lowercase().as_str());
        if sim > 0.80 {
            response.push(Entry::new(entry.clone(), sim, resp.clone()))
        }
    }
    HttpResponse::from(response)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[allow(unused_unsafe)] // will be needed in future versions
    unsafe {
        // std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,ecosnap")
        std::env::set_var("RUST_LOG", "info")
    };

    logger::init();

    HttpServer::new(|| {
        let data = json::from_str(include_str!("../../../data.json")).unwrap();

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
