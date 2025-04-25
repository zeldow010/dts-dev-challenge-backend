use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

use crate::tasks;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    Tasks(tasks::Error),
}

// region:  --- Froms

impl From<tasks::Error> for Error {
    fn from(val: tasks::Error) -> Self {
        Self::Tasks(val)
    }
}

// endregion:  --- Froms

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!("{:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(Arc::new(self));

		response
	}
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

// region:    --- Client Error

/// From the root error to the http status code and ClientError
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			Self::Tasks(tasks::Error::TaskNotFound) => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),
			Self::Tasks(tasks::Error::ValueTooShort {
				parameter: _,
				real_length: _,
				max_length: _
			}) => (StatusCode::BAD_REQUEST, ClientError::MALFORMED_REQUEST),
			// -- Fallback.
			_ => (
				StatusCode::SEE_OTHER,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	NOT_FOUND,
	MALFORMED_REQUEST,
	SERVICE_ERROR,
}
// endregion: --- Client Error