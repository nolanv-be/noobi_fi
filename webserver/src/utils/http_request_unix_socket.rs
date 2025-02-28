use axum::body::Body;
use http_body_util::BodyExt;
use hyper::{Method, Request, client::conn::http1};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use std::{path::PathBuf, string::FromUtf8Error};
use tokio::net::UnixStream;

#[derive(Debug)]
pub enum Error {
    SocketConnection(std::io::Error),
    Handhsake(hyper::Error),
    RequestBuild(axum::http::Error),
    RequestSend(hyper::Error),
    ResponseCollect(hyper::Error),
    BodyToUTF8(FromUtf8Error),
    ResponseNegative(String, String),
    ResponseParsing(serde_json::Error),
}

pub async fn send_get<T: DeserializeOwned>(
    socket_path: &str,
    uri: &str,
    body: Body,
) -> Result<T, Error> {
    let stream = TokioIo::new(
        UnixStream::connect(PathBuf::from(socket_path))
            .await
            .map_err(Error::SocketConnection)?,
    );

    let (mut sender, conn) = http1::handshake(stream).await.map_err(Error::Handhsake)?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection failed: {:?}", err);
        }
    });

    let request = Request::builder()
        .method(Method::GET)
        .header("Host", "localhost")
        .uri(uri)
        .body(body)
        .map_err(Error::RequestBuild)?;

    let response = sender
        .send_request(request)
        .await
        .map_err(Error::RequestSend)?;

    let is_success = response.status().is_success();
    let status_code = response.status().as_str().to_string();

    let body = response
        .collect()
        .await
        .map_err(Error::ResponseCollect)?
        .to_bytes();
    let body = String::from_utf8(body.to_vec()).map_err(Error::BodyToUTF8)?;

    if !is_success {
        return Err(Error::ResponseNegative(status_code, body));
    }

    serde_json::from_str(&body).map_err(Error::ResponseParsing)
}
