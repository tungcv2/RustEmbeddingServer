use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub name: String,
    pub directory: String,
    pub family: String,
    pub task: String,
    pub dimensions: usize,
    pub max_tokens: usize,
    pub supported_types: Vec<String>,
    pub default_model_file: String,
    pub files: Vec<String>,
    pub tokenizer_class: String,
    pub source_model: String,
    pub notes: String,
}

impl ModelMetadata {
    pub fn supports(&self, capability: &str) -> bool {
        self.supported_types.iter().any(|value| value == capability)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelInfo {
    pub name: String,
    pub dimensions: usize,
    pub max_tokens: usize,
    pub is_loaded: bool,
    pub supported_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAIModelData {
    pub id: String,
    #[serde(default = "default_object_model")]
    pub object: String,
    #[serde(default = "default_created")]
    pub created: i64,
    #[serde(default = "default_owned_by")]
    pub owned_by: String,
}

#[allow(dead_code)]
fn default_object_model() -> String {
    "model".to_string()
}

#[allow(dead_code)]
fn default_created() -> i64 {
    1_677_610_602
}

#[allow(dead_code)]
fn default_owned_by() -> String {
    "openai".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAIListModelResponse {
    #[serde(default = "default_object_list")]
    pub object: String,
    pub data: Vec<OpenAIModelData>,
}

#[allow(dead_code)]
fn default_object_list() -> String {
    "list".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAIEmbeddingData {
    #[serde(default = "default_embedding_object")]
    pub object: String,
    pub index: usize,
    pub embedding: Vec<f32>,
}

#[allow(dead_code)]
fn default_embedding_object() -> String {
    "embedding".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAIEmbeddingResponse {
    #[serde(default = "default_object_list")]
    pub object: String,
    pub data: Vec<OpenAIEmbeddingData>,
    pub model: String,
    pub usage: OpenAIUsage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAIEmbeddingRequest {
    pub model: String,
    pub input: serde_json::Value,
    pub user: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OllamaEmbeddingRequest {
    pub model: String,
    pub prompt: String,
    pub options: Option<serde_json::Value>,
    pub keep_alive: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OllamaEmbeddingResponse {
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenCountRequest {
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenCountResponse {
    pub count: usize,
    pub model: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SparseEmbeddingRequest {
    pub model: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct SparseEmbeddingData {
    pub index: usize,
    pub indices: Vec<usize>,
    pub values: Vec<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SparseEmbeddingResponse {
    pub model: String,
    pub data: Vec<SparseEmbeddingData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColBERTEmbeddingRequest {
    pub model: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColBERTEmbeddingData {
    pub index: usize,
    pub embeddings: Vec<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColBERTEmbeddingResponse {
    pub model: String,
    pub data: Vec<ColBERTEmbeddingData>,
    pub tokens: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RerankRequest {
    pub model: String,
    pub query: String,
    pub documents: Vec<String>,
    pub top_n: Option<usize>,
    pub return_documents: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RerankResult {
    pub index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct RerankResponse {
    pub model: String,
    pub results: Vec<RerankResult>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub models: Vec<ModelInfo>,
}
