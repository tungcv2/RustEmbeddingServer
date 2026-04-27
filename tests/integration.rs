use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use embedding_api_server::{
    backend::qwen3_rerank_prompt, config::AppConfig, error::AppError, registry::ModelRegistry,
    routes,
};
use http_body_util::BodyExt;
use std::{
    fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tower::util::ServiceExt;

fn temp_models_dir() -> PathBuf {
    static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let suffix = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "embedding-api-server-test-{}-{}-{}",
        std::process::id(),
        unique,
        suffix
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn write_model_with_types(dir: &PathBuf, name: &str, default_file: &str, supported_types: &[&str]) {
    let model_dir = dir.join(name);
    fs::create_dir_all(&model_dir).unwrap();
    fs::write(model_dir.join(default_file), b"test").unwrap();
    fs::write(
        model_dir.join("metadata.json"),
        serde_json::json!({
            "name": name,
            "directory": name,
            "family": "test",
            "task": "embedding",
            "dimensions": 8,
            "max_tokens": 16,
            "supported_types": supported_types,
            "default_model_file": default_file,
            "files": [default_file],
            "tokenizer_class": "TestTokenizer",
            "source_model": "test/source",
            "notes": "test"
        })
        .to_string(),
    )
    .unwrap();
}

fn write_model(dir: &PathBuf, name: &str, default_file: &str) {
    write_model_with_types(dir, name, default_file, &["embedding", "token_count"])
}

#[tokio::test]
async fn lists_models_from_metadata() {
    let models_dir = temp_models_dir();
    write_model(&models_dir, "model-a", "model.onnx");
    write_model(&models_dir, "model-b", "model.onnx");

    let config = AppConfig {
        models_dir: models_dir.clone(),
        default_model: Some("model-a".to_string()),
        model_ttl: Duration::from_secs(7200),
        bind_addr: "127.0.0.1:8000".parse().unwrap(),
    };
    let registry = ModelRegistry::discover(&config).await.unwrap();
    let app = routes::router(registry, config);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/models")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn serves_embedded_frontend() {
    let models_dir = temp_models_dir();
    write_model(&models_dir, "model-a", "model.onnx");

    let config = AppConfig {
        models_dir: models_dir.clone(),
        default_model: Some("model-a".to_string()),
        model_ttl: Duration::from_secs(7200),
        bind_addr: "127.0.0.1:8000".parse().unwrap(),
    };
    let registry = ModelRegistry::discover(&config).await.unwrap();
    let app = routes::router(registry, config);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();
    assert!(html.contains("Embedding API Server Studio"));
    assert!(html.contains("Metrics"));
    assert!(html.contains("/api/tokens/count"));
    assert!(html.contains("Raw JSON"));
}

#[tokio::test]
async fn exposes_metrics_for_recent_requests() {
    let models_dir = temp_models_dir();
    write_model(&models_dir, "model-a", "model.onnx");

    let config = AppConfig {
        models_dir: models_dir.clone(),
        default_model: Some("model-a".to_string()),
        model_ttl: Duration::from_secs(7200),
        bind_addr: "127.0.0.1:8000".parse().unwrap(),
    };
    let registry = ModelRegistry::discover(&config).await.unwrap();
    let app = routes::router(registry, config);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let metrics_response = app
        .oneshot(
            Request::builder()
                .uri("/api/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(metrics_response.status(), StatusCode::OK);
    let body = metrics_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["totals"]["calls"], 1);
    assert_eq!(json["totals"]["success"], 1);
    assert_eq!(json["recent"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn lazy_load_and_unload_expired_models() {
    let models_dir = temp_models_dir();
    write_model(&models_dir, "model-a", "model.onnx");

    let config = AppConfig {
        models_dir: models_dir.clone(),
        default_model: Some("model-a".to_string()),
        model_ttl: Duration::from_secs(0),
        bind_addr: "127.0.0.1:8000".parse().unwrap(),
    };
    let registry = ModelRegistry::discover(&config).await.unwrap();
    let response = registry
        .openai_embedding(embedding_api_server::models::OpenAIEmbeddingRequest {
            model: "model-a".to_string(),
            input: serde_json::Value::String("xin chao".to_string()),
            user: None,
        })
        .await
        .unwrap();

    assert_eq!(response.model, "model-a");
    assert_eq!(response.data.len(), 1);

    let unloaded = registry.unload_expired().await.unwrap();
    assert_eq!(unloaded, 1);
}

#[tokio::test]
async fn gpu_falls_back_to_cpu_when_unavailable() {
    let _guard = env_lock().lock().unwrap();
    std::env::set_var("USE_GPU", "true");
    std::env::set_var("GPU_AVAILABLE", "false");

    let models_dir = temp_models_dir();
    write_model(&models_dir, "model-a", "model.onnx");

    let config = AppConfig {
        models_dir,
        default_model: Some("model-a".to_string()),
        model_ttl: Duration::from_secs(7200),
        bind_addr: "127.0.0.1:8000".parse().unwrap(),
    };
    let registry = ModelRegistry::discover(&config).await.unwrap();
    let loaded = registry.ensure_loaded("model-a").await.unwrap();
    assert_eq!(
        loaded.backend,
        embedding_api_server::backend::BackendKind::Cpu
    );

    std::env::remove_var("USE_GPU");
    std::env::remove_var("GPU_AVAILABLE");
}

#[tokio::test]
async fn token_count_prefers_default_model() {
    let models_dir = temp_models_dir();
    write_model(&models_dir, "model-a", "model.onnx");
    write_model(&models_dir, "model-b", "model.onnx");

    let config = AppConfig {
        models_dir: models_dir.clone(),
        default_model: Some("model-b".to_string()),
        model_ttl: Duration::from_secs(7200),
        bind_addr: "127.0.0.1:8000".parse().unwrap(),
    };
    let registry = ModelRegistry::discover(&config).await.unwrap();

    let response = registry
        .token_count(embedding_api_server::models::TokenCountRequest {
            text: "xin chao".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(response.model, "model-b");
    assert_eq!(response.count, 2);
}

#[test]
fn qwen3_rerank_prompt_uses_expected_format() {
    let prompt = qwen3_rerank_prompt("what is rust", "a systems language");

    assert!(prompt.contains(
        "<Instruct>: Given a web search query, retrieve relevant passages that answer the query"
    ));
    assert!(prompt.contains("<Query>: what is rust"));
    assert!(prompt.contains("<Document>: a systems language"));
    assert!(!prompt.contains("<|im_start|>system"));
    assert!(!prompt.contains("<|im_end|>\n<|im_start|>assistant"));
}

#[tokio::test]
async fn embedding_requests_reject_incompatible_models() {
    let models_dir = temp_models_dir();
    write_model_with_types(
        &models_dir,
        "model-rerank",
        "model.onnx",
        &["rerank", "token_count"],
    );

    let config = AppConfig {
        models_dir: models_dir.clone(),
        default_model: Some("model-rerank".to_string()),
        model_ttl: Duration::from_secs(7200),
        bind_addr: "127.0.0.1:8000".parse().unwrap(),
    };
    let registry = ModelRegistry::discover(&config).await.unwrap();

    let error = registry
        .openai_embedding(embedding_api_server::models::OpenAIEmbeddingRequest {
            model: "model-rerank".to_string(),
            input: serde_json::Value::String("xin chao".to_string()),
            user: None,
        })
        .await
        .unwrap_err();

    assert!(
        matches!(error, AppError::BadRequest(message) if message.contains("does not support embedding"))
    );
}
