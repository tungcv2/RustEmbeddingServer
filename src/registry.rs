use crate::{
    backend::{BackendManager, LoadedModel},
    config::AppConfig,
    error::{AppError, AppResult},
    models::{
        ColBERTEmbeddingData, ColBERTEmbeddingRequest, ColBERTEmbeddingResponse, HealthResponse,
        ModelInfo, ModelMetadata, OllamaEmbeddingRequest, OllamaEmbeddingResponse,
        OpenAIEmbeddingData, OpenAIEmbeddingRequest, OpenAIEmbeddingResponse,
        OpenAIListModelResponse, OpenAIModelData, OpenAIUsage, RerankRequest, RerankResponse,
        RerankResult, SparseEmbeddingData, SparseEmbeddingRequest, SparseEmbeddingResponse,
        TokenCountRequest, TokenCountResponse,
    },
};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tokio::sync::{Mutex, RwLock};

#[derive(Debug)]
struct ModelRuntime {
    available: bool,
    loaded: Option<LoadedModel>,
    last_used: Option<Instant>,
    load_error: Option<String>,
}

#[derive(Debug)]
struct ModelEntry {
    metadata: ModelMetadata,
    runtime: Mutex<ModelRuntime>,
    #[allow(dead_code)]
    model_path: PathBuf,
}

#[derive(Clone)]
pub struct ModelRegistry {
    models_dir: PathBuf,
    default_model: Option<String>,
    model_ttl: std::time::Duration,
    backend: Arc<BackendManager>,
    models: Arc<RwLock<HashMap<String, Arc<ModelEntry>>>>,
}

impl ModelRegistry {
    pub async fn discover(config: &AppConfig) -> AppResult<Self> {
        let backend = Arc::new(BackendManager::new());
        let registry = Self {
            models_dir: config.models_dir.clone(),
            default_model: config.default_model.clone(),
            model_ttl: config.model_ttl,
            backend,
            models: Arc::new(RwLock::new(HashMap::new())),
        };
        registry.refresh().await?;
        Ok(registry)
    }

    pub fn spawn_reaper(&self) {
        let registry = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(error) = registry.unload_expired().await {
                    tracing::warn!(%error, "background unload failed");
                }
            }
        });
    }

    pub fn state(&self) -> RegistryState {
        RegistryState {
            models_dir: self.models_dir.clone(),
            default_model: self.default_model.clone(),
            model_ttl: self.model_ttl,
        }
    }

    pub async fn refresh(&self) -> AppResult<()> {
        let mut next = HashMap::new();
        if self.models_dir.exists() {
            for entry in std::fs::read_dir(&self.models_dir)? {
                let entry = entry?;
                if !entry.file_type()?.is_dir() {
                    continue;
                }
                let model_dir = entry.path();
                let metadata_path = model_dir.join("metadata.json");
                if !metadata_path.exists() {
                    continue;
                }
                let metadata = parse_metadata(&metadata_path)?;
                let model_key = metadata.name.clone();
                let load_error = validate_model_bundle(&model_dir, &metadata);
                next.insert(
                    model_key,
                    Arc::new(ModelEntry {
                        model_path: model_dir,
                        runtime: Mutex::new(ModelRuntime {
                            available: load_error.is_none(),
                            loaded: None,
                            last_used: None,
                            load_error,
                        }),
                        metadata,
                    }),
                );
            }
        }

        *self.models.write().await = next;
        Ok(())
    }

    pub async fn list_models(&self) -> Vec<ModelInfo> {
        let entries: Vec<Arc<ModelEntry>> = {
            let models = self.models.read().await;
            models.values().cloned().collect()
        };

        let mut out = Vec::with_capacity(entries.len());
        for entry in entries {
            let runtime = entry.runtime.lock().await;
            out.push(ModelInfo {
                name: entry.metadata.name.clone(),
                dimensions: entry.metadata.dimensions,
                max_tokens: entry.metadata.max_tokens,
                is_loaded: runtime.loaded.is_some(),
                supported_types: entry.metadata.supported_types.clone(),
            });
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        out
    }

    pub async fn openai_models(&self) -> Vec<OpenAIModelData> {
        let entries: Vec<Arc<ModelEntry>> = {
            let models = self.models.read().await;
            models.values().cloned().collect()
        };

        let mut out = Vec::with_capacity(entries.len());
        for entry in entries {
            out.push(OpenAIModelData {
                id: entry.metadata.name.clone(),
                object: "model".to_string(),
                created: 1_677_610_602,
                owned_by: "openai".to_string(),
            });
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }

    pub async fn health(&self) -> HealthResponse {
        HealthResponse {
            status: "ok".to_string(),
            models: self.list_models().await,
        }
    }

    pub async fn ensure_loaded(&self, model_name: &str) -> AppResult<LoadedModel> {
        let entry = self.model_entry(model_name).await?;
        {
            let runtime = entry.runtime.lock().await;
            if !runtime.available {
                return Err(AppError::BadRequest(
                    runtime
                        .load_error
                        .clone()
                        .unwrap_or_else(|| format!("model is not available: {}", model_name)),
                ));
            }
            if let Some(loaded) = runtime.loaded.clone() {
                drop(runtime);
                self.touch(&entry).await;
                return Ok(loaded);
            }
        }

        let loaded = self.backend.load_model(&entry.metadata).await?;
        {
            let mut runtime = entry.runtime.lock().await;
            runtime.available = true;
            runtime.loaded = Some(loaded.clone());
            runtime.last_used = Some(Instant::now());
            runtime.load_error = None;
        }
        Ok(loaded)
    }

    pub async fn unload_expired(&self) -> AppResult<usize> {
        let entries: Vec<Arc<ModelEntry>> = {
            let models = self.models.read().await;
            models.values().cloned().collect()
        };

        let mut unloaded = 0usize;
        for entry in entries {
            let maybe_loaded = {
                let mut runtime = entry.runtime.lock().await;
                match (runtime.loaded.take(), runtime.last_used) {
                    (Some(loaded), Some(last_used)) if last_used.elapsed() >= self.model_ttl => {
                        runtime.last_used = None;
                        Some(loaded)
                    }
                    (loaded, last_used) => {
                        runtime.loaded = loaded;
                        runtime.last_used = last_used;
                        None
                    }
                }
            };

            if let Some(loaded) = maybe_loaded {
                self.backend.unload_model(loaded).await?;
                unloaded += 1;
            }
        }

        Ok(unloaded)
    }

    async fn touch(&self, entry: &Arc<ModelEntry>) {
        let mut runtime = entry.runtime.lock().await;
        runtime.last_used = Some(Instant::now());
    }

    async fn model_entry(&self, model_name: &str) -> AppResult<Arc<ModelEntry>> {
        let models = self.models.read().await;
        models
            .get(model_name)
            .cloned()
            .ok_or_else(|| AppError::ModelNotFound(model_name.to_string()))
    }

    async fn entry_for_capability(
        &self,
        model_name: &str,
        capability: &str,
    ) -> AppResult<Arc<ModelEntry>> {
        let entry = self.model_entry(model_name).await?;
        if !entry.metadata.supports(capability) {
            return Err(AppError::BadRequest(format!(
                "model {model_name} does not support {capability}"
            )));
        }
        Ok(entry)
    }

    async fn preferred_model_for_capability(&self, capability: &str) -> AppResult<String> {
        let entries: Vec<Arc<ModelEntry>> = {
            let models = self.models.read().await;
            models.values().cloned().collect()
        };

        if let Some(default_model) = self.default_model_name() {
            if let Some(entry) = entries
                .iter()
                .find(|entry| entry.metadata.name == default_model)
            {
                let runtime = entry.runtime.lock().await;
                if runtime.available && entry.metadata.supports(capability) {
                    return Ok(default_model);
                }
            }
        }

        let mut candidates: Vec<String> = Vec::new();
        for entry in entries {
            let runtime = entry.runtime.lock().await;
            if runtime.available && entry.metadata.supports(capability) {
                candidates.push(entry.metadata.name.clone());
            }
        }
        candidates.sort();
        candidates.into_iter().next().ok_or_else(|| {
            AppError::BadRequest(format!("no available model supports {capability}"))
        })
    }

    fn default_model_name(&self) -> Option<String> {
        self.default_model.clone()
    }

    pub async fn openai_embedding(
        &self,
        request: OpenAIEmbeddingRequest,
    ) -> AppResult<OpenAIEmbeddingResponse> {
        let entry = self
            .entry_for_capability(&request.model, "embedding")
            .await?;
        let model_path = entry.model_path.clone();
        let metadata = entry.metadata.clone();
        self.ensure_loaded(&request.model).await?;
        let inputs = parse_text_inputs(request.input)?;
        let embeddings = crate::backend::embed_texts(
            &model_path,
            &metadata.default_model_file,
            &inputs,
            metadata.dimensions,
            metadata.max_tokens,
        )
        .await?;
        let mut data = Vec::with_capacity(inputs.len());
        let mut total_tokens = 0usize;

        for (index, (input, embedding)) in inputs.iter().zip(embeddings).enumerate() {
            total_tokens += crate::backend::token_count(input);
            data.push(OpenAIEmbeddingData {
                object: "embedding".to_string(),
                index,
                embedding,
            });
        }

        Ok(OpenAIEmbeddingResponse {
            object: "list".to_string(),
            data,
            model: request.model,
            usage: OpenAIUsage {
                prompt_tokens: total_tokens,
                total_tokens,
            },
        })
    }

    pub async fn ollama_embedding(
        &self,
        request: OllamaEmbeddingRequest,
    ) -> AppResult<OllamaEmbeddingResponse> {
        let entry = self
            .entry_for_capability(&request.model, "embedding")
            .await?;
        let model_path = entry.model_path.clone();
        let metadata = entry.metadata.clone();
        self.ensure_loaded(&request.model).await?;
        let embeddings = crate::backend::embed_texts(
            &model_path,
            &metadata.default_model_file,
            &[request.prompt.clone()],
            metadata.dimensions,
            metadata.max_tokens,
        )
        .await?;
        Ok(OllamaEmbeddingResponse {
            embedding: embeddings.into_iter().next().unwrap_or_default(),
        })
    }

    pub async fn token_count(&self, request: TokenCountRequest) -> AppResult<TokenCountResponse> {
        let model = self.preferred_model_for_capability("token_count").await?;
        Ok(TokenCountResponse {
            count: crate::backend::token_count(&request.text),
            model,
        })
    }

    pub async fn sparse_embedding(
        &self,
        request: SparseEmbeddingRequest,
    ) -> AppResult<SparseEmbeddingResponse> {
        let entry = self
            .entry_for_capability(&request.model, "sparse_embedding")
            .await?;
        let metadata = entry.metadata.clone();
        self.ensure_loaded(&request.model).await?;
        let inputs = parse_text_inputs(request.input)?;
        let mut data = Vec::with_capacity(inputs.len());

        for (index, input) in inputs.iter().enumerate() {
            let tokens: Vec<&str> = input.split_whitespace().collect();
            let mut indices = Vec::with_capacity(tokens.len());
            let mut values = Vec::with_capacity(tokens.len());
            for (position, token) in tokens.iter().enumerate() {
                indices.push(position);
                values.push(hash_to_unit_float(token) * (metadata.dimensions as f32 / 1024.0));
            }
            data.push(SparseEmbeddingData {
                index,
                indices,
                values,
            });
        }

        Ok(SparseEmbeddingResponse {
            model: request.model,
            data,
        })
    }

    pub async fn colbert_embedding(
        &self,
        request: ColBERTEmbeddingRequest,
    ) -> AppResult<ColBERTEmbeddingResponse> {
        let entry = self
            .entry_for_capability(&request.model, "colbert_embedding")
            .await?;
        let metadata = entry.metadata.clone();
        self.ensure_loaded(&request.model).await?;
        let inputs = parse_text_inputs(request.input)?;
        let mut data = Vec::with_capacity(inputs.len());
        let mut tokens_out = Vec::with_capacity(inputs.len());

        for (index, input) in inputs.iter().enumerate() {
            let token_list: Vec<String> = input
                .split_whitespace()
                .map(|value| value.to_string())
                .collect();
            let embeddings = token_list
                .iter()
                .map(|token| {
                    crate::backend::deterministic_embedding(token, metadata.dimensions.min(32))
                })
                .collect();
            tokens_out.push(token_list);
            data.push(ColBERTEmbeddingData { index, embeddings });
        }

        Ok(ColBERTEmbeddingResponse {
            model: request.model,
            data,
            tokens: Some(tokens_out),
        })
    }

    pub async fn rerank(&self, request: RerankRequest) -> AppResult<RerankResponse> {
        let entry = self.entry_for_capability(&request.model, "rerank").await?;
        let model_path = entry.model_path.clone();
        let metadata = entry.metadata.clone();
        self.ensure_loaded(&request.model).await?;

        let scores = crate::backend::rerank_documents(
            &metadata,
            &model_path,
            &metadata.default_model_file,
            &request.query,
            &request.documents,
            metadata.max_tokens,
        )
        .await?;

        let return_documents = request.return_documents.unwrap_or(true);
        let mut results: Vec<RerankResult> = request
            .documents
            .iter()
            .cloned()
            .enumerate()
            .zip(scores)
            .map(|((index, document), score)| RerankResult {
                index,
                document: return_documents.then_some(document),
                score,
            })
            .collect();
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if let Some(top_n) = request.top_n {
            results.truncate(top_n);
        }
        Ok(RerankResponse {
            model: request.model,
            results,
        })
    }

    pub async fn openai_list_models(&self) -> AppResult<OpenAIListModelResponse> {
        Ok(OpenAIListModelResponse {
            object: "list".to_string(),
            data: self.openai_models().await,
        })
    }
}

#[derive(Clone)]
pub struct RegistryState {
    pub models_dir: PathBuf,
    pub default_model: Option<String>,
    pub model_ttl: std::time::Duration,
}

fn parse_metadata(path: &Path) -> AppResult<ModelMetadata> {
    let content = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

fn parse_text_inputs(value: Value) -> AppResult<Vec<String>> {
    match value {
        Value::String(text) => Ok(vec![text]),
        Value::Array(values) => values
            .into_iter()
            .map(|value| match value {
                Value::String(text) => Ok(text),
                _ => Err(AppError::BadRequest(
                    "input array must contain only strings".to_string(),
                )),
            })
            .collect(),
        _ => Err(AppError::BadRequest(
            "input must be a string or array of strings".to_string(),
        )),
    }
}

fn validate_model_bundle(model_dir: &Path, metadata: &ModelMetadata) -> Option<String> {
    let mut missing = Vec::new();

    for file_name in &metadata.files {
        if !model_dir.join(file_name).exists() {
            missing.push(file_name.clone());
        }
    }

    if !model_dir.join(&metadata.default_model_file).exists()
        && !missing.contains(&metadata.default_model_file)
    {
        missing.push(metadata.default_model_file.clone());
    }

    if missing.is_empty() {
        None
    } else {
        Some(format!("missing model files: {}", missing.join(", ")))
    }
}

fn hash_to_unit_float(value: &str) -> f32 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() % 10_000) as f32 / 10_000.0
}
