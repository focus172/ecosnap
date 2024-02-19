use crate::State;
use actix_web::{error, post, web, Error, HttpResponse};
use base64::Engine;
use resu::{Context, ResultExt};
use serde::{Deserialize, Serialize};
use std::{fmt, io::Cursor};

const MAX_SIZE: usize = const { 256 * 1024 * 1024 }; // 256M

#[derive(Debug, Default, Serialize, Deserialize)]
struct Request {
    // api_key: String,
    // version: String,
    /// The query
    // query: Query,
    /// The data section, contains base 64 image when the query is an image
    data: String,
}

#[post("/v2/search")]
pub async fn search(payload: web::Payload, state: web::Data<State>) -> Result<HttpResponse, Error> {
    log::info!("got new request");
    let Ok(read) = payload.to_bytes_limited(MAX_SIZE).await else {
        return Err(error::ErrorBadRequest("overflow"));
    };
    let Ok(body) = read else { panic!() };

    // body is loaded, now we can deserialize serde-json
    let reqs = json::from_slice::<Request>(&body)?;
    let data = preprocess(reqs.data).unwrap();

    // log::info!("got thing. ready to sending req");

    let resp = match crate::google::call(&state.key, data).await {
        Ok(a) => a,
        Err(b) => {
            return Err(error::ErrorBadRequest(format!("e: {}", b)));
        }
    };
    log::info!("resp: {:?}", resp);

    let logos = resp.logos();

    Ok(HttpResponse::Ok().json(logos))
}

#[derive(Debug)]
enum ImageError {
    DecodeError,
    ParseError,
}
impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::DecodeError => f.write_str("could not understand input as base64"),
            ImageError::ParseError => f.write_str("input does not appear to be a image"),
        }
    }
}
impl Context for ImageError {}

/// Takes a image in base64 and preforms processing on it
///
/// In this case it is compression to make it smaller
fn preprocess(data: String) -> resu::Result<String, ImageError> {
    let bytes = base64::prelude::BASE64_STANDARD
        .decode(data)
        .change_context(ImageError::DecodeError)?;

    let image = image::io::Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .expect("cursors nevery fail")
        .decode()
        .change_context(ImageError::ParseError)?;

    let image = image.resize(320, 320, image::imageops::FilterType::Nearest);

    Ok(base64::prelude::BASE64_STANDARD.encode(image.as_bytes()))
}
