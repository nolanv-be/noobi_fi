mod assets;
mod incus;
pub mod utils;

pub use assets::serve_all_assets;
use incus::serve_all_incus;

pub fn serve_all_applications() -> axum::Router {
    axum::Router::new()
        .merge(serve_all_incus())
        .merge(serve_all_assets())
}
