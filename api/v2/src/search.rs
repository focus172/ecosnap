use crate::State;
use actix_web::{error, post, web, Error, HttpResponse};
use base64::Engine;
use resu::{Context, ResultExt};
use serde::{Deserialize, Serialize};
use std::{fmt, io};
use tokio::process::Command;

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

    let key = get_key().await;
    let resp = match crate::google::call(key.trim_end(), data).await {
        Ok(a) => a,
        Err(e) => {
            log::info!("{:?}", e);
            return Err(error::ErrorBadRequest(e));
        }
    };
    log::info!("resp: {:?}", resp);

    let logos = resp.logos();
    let response = crate::make_matches(&state, &logos).await;

    Ok(HttpResponse::from(response))
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

    let mut image = image::io::Reader::new(io::Cursor::new(bytes));
    image.set_format(image::ImageFormat::Png);
    let image = image
        .decode()
        .change_context(ImageError::ParseError)
        .attach_printable("file should be a png")?;
    let image = image.resize(320, 320, image::imageops::FilterType::Nearest);

    let mut buf = Vec::<u8>::new();
    {
        let mut w = io::BufWriter::new(io::Cursor::new(&mut buf));
        image.write_to(&mut w, image::ImageFormat::Png).unwrap();
    }

    Ok(base64::prelude::BASE64_STANDARD.encode(buf))
}

async fn get_key() -> String {
    let data = Command::new("gcloud")
        .arg("auth")
        .arg("print-access-token")
        .output()
        .await
        .unwrap()
        .stdout;
    String::from_utf8(data).unwrap()
}
