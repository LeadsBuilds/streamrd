use axum::{
    routing::get,
    Router
};

pub fn define_routes(index: Router) -> Router {
   index
   .route("/", get(|| async { "Hello, World!" }))
   .route("/worlds", get(|| async { "Hello, Worlds!" }))
}