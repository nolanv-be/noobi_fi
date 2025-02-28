use axum::body::Body;
use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::utils::http_request_unix_socket::send_get;

const INCUS_UNIX_PATH: &str = "/run/incus/unix.socket";

#[derive(Serialize, Deserialize)]
struct IncusResponse {
    metadata: Vec<String>,
    status: String,
    status_code: Number,
    r#type: String,
}

pub async fn get_incus_version() -> Result<String, String> {
    let start = std::time::Instant::now();
    let incus_response: IncusResponse = send_get(INCUS_UNIX_PATH, "http://incus/", Body::empty())
        .await
        .map_err(|e| format!("{:#?}", e))?;
    let duration = start.elapsed();
    dbg!("handshake: ", duration.as_micros());

    incus_response
        .metadata
        .first()
        .ok_or("Failed to get first metadata".into())
        .cloned()
}
