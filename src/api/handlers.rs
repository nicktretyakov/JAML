use crate::api::dto::*;
use crate::api::state::AppState;
use crate::domain::ContentKind;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<ApiError>)>;

fn map_err(e: crate::core::CmsError) -> (StatusCode, Json<ApiError>) {
    let status = match &e {
        crate::core::CmsError::NotFound(_) => StatusCode::NOT_FOUND,
        crate::core::CmsError::Validation(_) => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (
        status,
        Json(ApiError {
            error: e.to_string(),
        }),
    )
}

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "service": "solid_ssg_cms" }))
}

pub async fn list_content(State(state): State<Arc<AppState>>) -> ApiResult<Vec<ContentResponse>> {
    let items = state.content.list().await.map_err(map_err)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn get_content(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> ApiResult<ContentResponse> {
    let item = state
        .content
        .get(&slug)
        .await
        .map_err(map_err)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError {
                    error: format!("Content '{}' not found", slug),
                }),
            )
        })?;
    Ok(Json(item.into()))
}

pub async fn create_content(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateContentRequest>,
) -> ApiResult<ContentResponse> {
    let kind = match req.kind.as_deref() {
        Some("page") => ContentKind::Page,
        Some("product") => ContentKind::Product,
        Some(other) => ContentKind::Custom(other.to_string()),
        None => ContentKind::Post,
    };
    let author = req.author.unwrap_or_else(|| "anonymous".into());
    let tags = req.tags.unwrap_or_default();

    let item = state
        .content
        .create(req.slug, req.title, req.body, kind, author, tags)
        .await
        .map_err(map_err)?;
    Ok(Json(item.into()))
}

pub async fn update_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateContentRequest>,
) -> ApiResult<ContentResponse> {
    let item = state
        .content
        .update(id, req.title, req.body, req.tags)
        .await
        .map_err(map_err)?;
    Ok(Json(item.into()))
}

pub async fn publish_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<ContentResponse> {
    let item = state.content.publish(id).await.map_err(map_err)?;
    Ok(Json(item.into()))
}

pub async fn delete_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ApiError>)> {
    state.content.delete(id).await.map_err(map_err)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn trigger_build(State(state): State<Arc<AppState>>) -> ApiResult<BuildResponse> {
    let manifest = state
        .builder
        .build(&state.config)
        .await
        .map_err(map_err)?;
    Ok(Json(BuildResponse {
        success: true,
        pages: manifest.pages.len(),
        duration_ms: manifest.build_duration_ms,
        generated_at: manifest.generated_at.to_rfc3339(),
    }))
}

pub async fn get_config(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "title": state.config.title,
        "description": state.config.description,
        "base_url": state.config.base_url,
        "parallel_workers": state.config.parallel_workers,
        "output_dir": state.config.output_dir,
    }))
}
