use serde::Serialize;
use worker::*;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<&'static str>,
}

pub fn json_error(status: u16, code: &'static str, message: impl Into<String>) -> Result<Response> {
    Ok(Response::from_json(&ErrorBody {
        error: ErrorDetail {
            code,
            message: message.into(),
            field: None,
        },
    })?
    .with_status(status))
}

pub fn method_not_allowed() -> Result<Response> {
    json_error(405, "METHOD_NOT_ALLOWED", "method is not allowed")
}
