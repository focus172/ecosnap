use std::{collections::HashMap, fmt};

use json::json;
use resu::{Context, ResultExt};
use serde::Deserialize;

use crate::GOOGLE_PROJECT_ID;

#[derive(Debug)]
pub enum ApiError {
    SendError,
    ResponseError,
    ParseError(json::Value),
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::SendError => f.write_str("failed to send request."),
            ApiError::ResponseError => f.write_str("failed to understand response"),
            ApiError::ParseError(v) => write!(f, "failed to serialize value: {}", v),
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
        .change_context(ApiError::SendError)?
        .json::<json::Value>()
        .await
        .change_context(ApiError::ResponseError)?;

    json::from_value(value.clone()).change_context(ApiError::ParseError(value))
}

/// Response from google API
///
/// The example can be forund here
/// https://cloud.google.com/vision/docs/detecting-logos#vision_logo_detection-drest
#[derive(Debug, Default, Deserialize)]
pub struct GoogleApiResponse {
    responses: Vec<HashMap<String, Annotation>>,
}

impl GoogleApiResponse {
    pub fn logos(&self) -> Vec<String> {
        self.responses
            .iter()
            .flat_map(|m| m.get("logoAnnotations"))
            .map(|annotation| annotation.description.clone())
            .collect()
    }
}

#[derive(Debug, Default, Deserialize)]
struct Annotation {
    mid: String,
    description: String,
    score: f32,
    #[serde(rename(serialize = "boundingPoly"))]
    poly: Vec<Vert>,
}

/// A Point on an image, Desrializes to 0 when it is missing
#[derive(Debug, Default, Deserialize)]
struct Vert {
    #[serde(default)]
    x: usize,
    #[serde(default)]
    y: usize,
}
