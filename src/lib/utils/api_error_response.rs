use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

/// `APIErrorResponse is the start way to return error message.
pub struct APIErrorResponse {
    status: StatusCode,
    message: String,
}
impl APIErrorResponse {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self { status, message }
    }
}

pub const INTERNAL_SERVER_ERROR_RESPONSE: APIErrorResponse = APIErrorResponse::new(
    StatusCode::INTERNAL_SERVER_ERROR,
    String::from("Internal Server Error"),
);

impl Serialize for APIErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("APIErrorResponse", 2)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl IntoResponse for APIErrorResponse {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

/// `APIResult is returned by services`
pub type APIResult<T> = Result<T, APIErrorResponse>;

impl From<sqlx::Error> for APIErrorResponse {
    fn from(_: sqlx::Error) -> Self {
        INTERNAL_SERVER_ERROR_RESPONSE
    }
}
