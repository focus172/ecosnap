use std::fmt;

use json::json;
use resu::{Context, ResultExt};
use serde::Deserialize;

use crate::GOOGLE_PROJECT_ID;

#[derive(Debug)]
pub enum ApiError {
    Send,
    Response,
    Parse(json::Value),
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Send => f.write_str("failed to send request."),
            ApiError::Response => f.write_str("failed to understand response"),
            ApiError::Parse(v) => write!(
                f,
                "failed to serialize value: \n {}",
                json::to_string_pretty(v).unwrap()
            ),
        }
    }
}
impl Context for ApiError {}

pub async fn call(key: &str, data: String) -> resu::Result<GoogleApiResponse, ApiError> {
    let value = reqwest::Client::new()
        .post("https://vision.googleapis.com/v1/images:annotate")
        .header("Authorization", format!("Bearer {}", key))
        .header("x-goog-user-project", GOOGLE_PROJECT_ID)
        .header("Content-Type", "application/json; charset=utf-8")
        .json(&json!({
          "requests": [
            {
              "image": {
                "content": data,
              },
              "features": [
                {
                  "type": "LOGO_DETECTION"
                },
              ]
            }
          ]
        }))
        .send()
        .await
        .change_context(ApiError::Send)?
        .json::<json::Value>()
        .await
        .change_context(ApiError::Response)?;

    json::from_value(value.clone()).change_context(ApiError::Parse(value))
}

/// Response from google API
///
/// The example can be forund here
/// https://cloud.google.com/vision/docs/detecting-logos#vision_logo_detection-drest
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum GoogleApiResponse {
    Responses(Vec<Response>),
    Error(GoogleError),
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GoogleError {
    code: usize,
    details: json::Value,
    message: String,
    status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub enum Response {
    LogoAnnotations(Vec<Annotation>),
    Error(json::Value),
}

impl GoogleApiResponse {
    pub fn logos(&self) -> Vec<String> {
        match self {
            GoogleApiResponse::Responses(r) => r
                .iter()
                .filter_map(|resp| match resp {
                    Response::LogoAnnotations(a) => Some(a),
                    Response::Error(e) => {
                        log::error!("{:?}", e);
                        None
                    }
                })
                .flatten()
                .map(|annotation| annotation.description.clone())
                .collect(),
            GoogleApiResponse::Error(e) => {
                log::warn!("{e:#?}");
                vec![]
            }
        }
    }
}

#[allow(unused)] // all feilds are in the struct
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Annotation {
    mid: String,
    description: String,
    score: f32,
    bounding_poly: Poly,
}

#[allow(unused)] // all feilds are in the struct
#[derive(Debug, Default, Deserialize)]
pub struct Poly {
    vertices: [Vert; 4],
}

/// A Point on an image, Desrializes to 0 when it is missing
#[allow(unused)]
#[derive(Debug, Default, Deserialize)]
struct Vert {
    #[serde(default)]
    x: usize,
    #[serde(default)]
    y: usize,
}
