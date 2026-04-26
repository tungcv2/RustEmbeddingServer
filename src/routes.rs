use crate::{config::AppConfig, error::AppError, frontend, registry::ModelRegistry};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
pub struct AppState {
    pub registry: ModelRegistry,
    pub config: AppConfig,
}

pub fn router(registry: ModelRegistry, config: AppConfig) -> Router {
    let state = AppState { registry, config };
    Router::new()
        .route("/", get(frontend::index))
        .route("/health", get(health))
        .route("/v1/models", get(list_models))
        .route("/v1/embeddings", post(openai_embeddings))
        .route("/api/embeddings", post(ollama_embeddings))
        .route("/api/embeddings/sparse", post(sparse_embeddings))
        .route("/api/embeddings/colbert", post(colbert_embeddings))
        .route("/api/rerank", post(rerank))
        .route("/api/tokens/count", post(token_count))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

pub async fn health(
    State(state): State<AppState>,
) -> Result<Json<crate::models::HealthResponse>, AppError> {
    Ok(Json(state.registry.health().await))
}

pub async fn list_models(
    State(state): State<AppState>,
) -> Result<Json<crate::models::OpenAIListModelResponse>, AppError> {
    Ok(Json(state.registry.openai_list_models().await?))
}

pub async fn openai_embeddings(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::OpenAIEmbeddingRequest>,
) -> Result<Json<crate::models::OpenAIEmbeddingResponse>, AppError> {
    Ok(Json(state.registry.openai_embedding(payload).await?))
}

pub async fn ollama_embeddings(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::OllamaEmbeddingRequest>,
) -> Result<Json<crate::models::OllamaEmbeddingResponse>, AppError> {
    Ok(Json(state.registry.ollama_embedding(payload).await?))
}

pub async fn token_count(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::TokenCountRequest>,
) -> Result<Json<crate::models::TokenCountResponse>, AppError> {
    Ok(Json(state.registry.token_count(payload).await?))
}

pub async fn sparse_embeddings(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::SparseEmbeddingRequest>,
) -> Result<Json<crate::models::SparseEmbeddingResponse>, AppError> {
    Ok(Json(state.registry.sparse_embedding(payload).await?))
}

pub async fn colbert_embeddings(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::ColBERTEmbeddingRequest>,
) -> Result<Json<crate::models::ColBERTEmbeddingResponse>, AppError> {
    Ok(Json(state.registry.colbert_embedding(payload).await?))
}

pub async fn rerank(
    State(state): State<AppState>,
    Json(payload): Json<crate::models::RerankRequest>,
) -> Result<Json<crate::models::RerankResponse>, AppError> {
    Ok(Json(state.registry.rerank(payload).await?))
}

pub async fn warmup(state: AppState) -> Result<(), AppError> {
    let registry = state.registry.clone();
    let ttl = state.config.model_ttl;
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            if let Err(error) = registry.unload_expired().await {
                tracing::warn!(%error, "background unload failed");
            }
            let _ = ttl;
        }
    });
    Ok(())
}

pub async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_FOUND, Json(json!({"error": "not found"})))
}
