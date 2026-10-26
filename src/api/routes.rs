use crate::api::handlers::*;
use crate::api::state::AppState;
use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn create_router(state: Arc<AppState>) -> Router {
    let api = Router::new()
        .route("/health", get(health))
        .route("/config", get(get_config))
        .route("/content", get(list_content).post(create_content))
        .route("/content/by-slug/:slug", get(get_content))
        .route("/content/by-id/:id", put(update_content))
        .route("/content/by-id/:id", axum::routing::delete(delete_content))
        .route("/content/by-id/:id/publish", post(publish_content))
        .route("/build", post(trigger_build));

    Router::new()
        .nest("/api", api)
        .nest_service("/site", ServeDir::new(state.config.output_dir.clone()))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/", ServeDir::new("public"))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
