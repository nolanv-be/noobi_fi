use tower_http::services::ServeDir;

pub fn serve_all_assets() -> axum::Router {
    axum::Router::new().nest_service("/assets", ServeDir::new("assets"))
}
