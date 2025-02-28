mod index;

use axum::routing::get;
use maud::{Markup, html};

use crate::utils;

pub fn serve_all_incus() -> axum::Router {
    axum::Router::new().route("/incus", get(serve_index))
}

async fn serve_index() -> Markup {
    let version = index::get_incus_version().await;
    html! {
        (utils::html::make_header_html("noobi.fi - incus"))
        main class="container" {
            @if version.is_ok() {
                h1 {"Success: " (version.unwrap())}
            }
        }
    }
}
