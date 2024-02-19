#![feature(lazy_cell)]

use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::response::{QueryResponse, Thing};

pub const API_VERSION: &str = "0.2.0";
const MAX_SIZE: usize = 262_144; // max payload size is 256k

mod response;

#[derive(Debug)]
pub struct State {
    data: HashMap<String, HashMap<String, usize>>,
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

#[derive(Serialize, Deserialize)]
struct Request {
    api_key: String,
    version: String,
    /// The query
    query: Query,
    /// The data section, contains base 64 image when the query is an image
    data: String,
}

#[post("/v2/search")]
async fn search(mut payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = json::from_slice::<Request>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
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

        App::new()
            .app_data(web::Data::new(State { data }))
            .service(search)
            .service(get)
            .service(find)
    })
    .bind(("127.0.0.1", 6699))?
    .run()
    .await
}
