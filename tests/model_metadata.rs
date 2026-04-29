use embedding_api_server::models::ModelMetadata;
use std::{fs, path::Path};

#[test]
fn jina_embeddings_v5_text_small_clustering_metadata_is_valid() {
    let model_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("AI_Models/jina-embeddings-v5-text-small-clustering");
    let metadata_path = model_dir.join("metadata.json");
    let metadata: ModelMetadata = serde_json::from_str(&fs::read_to_string(&metadata_path).unwrap())
        .unwrap();

    assert_eq!(metadata.name, "jina-embeddings-v5-text-small-clustering");
    assert_eq!(metadata.directory, "jina-embeddings-v5-text-small-clustering");
    assert_eq!(metadata.family, "jina-embeddings-v5-text");
    assert_eq!(metadata.task, "embedding");
    assert_eq!(metadata.dimensions, 1024);
    assert_eq!(metadata.max_tokens, 32768);
    assert_eq!(metadata.default_model_file, "model.onnx");
    assert_eq!(
        metadata.supported_types,
        vec!["embedding".to_string(), "token_count".to_string()]
    );
    assert_eq!(metadata.tokenizer_class, "Qwen2Tokenizer");
    assert_eq!(
        metadata.source_model,
        "jinaai/jina-embeddings-v5-text-small-clustering"
    );

    for file_name in &metadata.files {
        assert!(model_dir.join(file_name).exists(), "missing file: {file_name}");
    }
    assert!(model_dir.join(&metadata.default_model_file).exists());
}
