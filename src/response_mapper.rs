use std::sync::Arc;

use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::error::Error;

pub async fn mw_response_map(res: Response) -> Response {
    debug!("{:<12} - mw_reponse_map", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let web_error = res.extensions().get::<Arc<Error>>().map(Arc::as_ref);
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            debug!("CLIENT ERROR BODY:\n{client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    debug!("\n");

    error_response.unwrap_or(res)
}
