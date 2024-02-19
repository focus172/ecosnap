#![feature(lazy_cell, inline_const)]

use crate::response::{QueryResponse, Thing};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");
const GOOGLE_PROJECT_ID: &str = "hidden-sunlight-414619";

// let key - "$(gcloud auth print-access-token)"),

mod google;
mod response;
mod search;

#[derive(Debug)]
pub struct State {
    pub data: HashMap<String, HashMap<String, usize>>,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Query {
    Name(String),
    Image,
}

impl From<String> for Query {
    fn from(value: String) -> Self {
        Query::Name(value)
    }
}

#[derive(Deserialize)]
struct Name {
    name: String,
}

/// Search based on exact matches
#[get("/v2/get/{name}")]
async fn get(path: web::Path<Name>, state: web::Data<State>) -> impl Responder {
    let name = &path.name;
    log::info!("Got input: {}", path.name);

    let mut response = QueryResponse::query(name.clone());

    if let Some(value) = state.data.get(&path.name) {
        response.push(Thing::new(path.name.clone(), 1.0, value.clone()))
    }

    HttpResponse::from(response)
}

/// Search based using fuzzy finder
#[get("/v2/find/{name}")]
async fn find(path: web::Path<Name>, state: web::Data<State>) -> impl Responder {
    let name = &path.name;
    let mut response = QueryResponse::query(name.clone());
    for (entry, resp) in state.data.iter() {
        let sim = strsim::jaro_winkler(name.to_lowercase().as_str(), entry.to_lowercase().as_str());
        if sim > 0.75 {
            response.push(Thing::new(entry.clone(), sim, resp.clone()))
        }
    }
    HttpResponse::from(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,ecosnap");
    logger::init();

    HttpServer::new(|| {
        let data = json::from_str(include_str!("../../data.json")).unwrap();
        let key = include_str!("../access-token.priv").trim().to_owned();

        App::new()
            .app_data(web::Data::new(State { data, key }))
            .service(search::search)
            .service(get)
            .service(find)
    })
    .bind(("127.0.0.1", 6699))?
    .run()
    .await
}
