use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[allow(dead_code)]
pub enum CustomError {
    WorkerError,
    ParseError,
}

/// Any response that is fallible needs to impl `IntoResponse` and be able to
/// convert into a valid response
/// Reference on building a response:
/// https://docs.rs/axum/0.6.16/axum/response/index.html
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            "Example of a response returning an error",
        )
            .into_response()
    }
}

pub async fn error_example() -> Result<String, CustomError> {
    let should_succeed: bool = false;
    match should_succeed {
        true => Ok("You can never see this".into()),
        false => Err(CustomError::WorkerError),
    }
}
