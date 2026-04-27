use crate::{
    error::{AppError, AppResult},
    models::ModelMetadata,
};
use half::f16;
use ort::{
    memory::Allocator,
    session::{builder::GraphOptimizationLevel, Session},
    value::{DynValue, Tensor},
};
use serde::{Deserialize, Serialize};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};
use tokenizers::Tokenizer;
use tokio::task;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendKind {
    Gpu,
    Cpu,
}

#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub backend: BackendKind,
    pub model_name: String,
    pub dimensions: usize,
}

#[derive(Debug)]
struct RustModelRuntime {
    tokenizer: Arc<Tokenizer>,
    session: Mutex<Session>,
    input_names: Vec<String>,
}

static RUST_RUNTIME_CACHE: OnceLock<Mutex<HashMap<String, Arc<RustModelRuntime>>>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct BackendManager {
    pub prefer_gpu: bool,
}

impl BackendManager {
    pub fn new() -> Self {
        Self {
            prefer_gpu: std::env::var("USE_GPU")
                .map(|v| v != "0" && v.to_lowercase() != "false")
                .unwrap_or(true),
        }
    }

    pub async fn load_model(&self, metadata: &ModelMetadata) -> AppResult<LoadedModel> {
        let backend = if self.prefer_gpu && gpu_available() {
            BackendKind::Gpu
        } else {
            BackendKind::Cpu
        };

        Ok(LoadedModel {
            backend,
            model_name: metadata.name.clone(),
            dimensions: metadata.dimensions,
        })
    }

    pub async fn unload_model(&self, _loaded: LoadedModel) -> AppResult<()> {
        Ok(())
    }
}

pub fn gpu_available() -> bool {
    std::env::var("GPU_AVAILABLE")
        .map(|v| v != "0" && v.to_lowercase() != "false")
        .unwrap_or(false)
}

pub fn deterministic_embedding(input: &str, dimensions: usize) -> Vec<f32> {
    let mut output = vec![0.0; dimensions.max(1)];
    let size = output.len();
    for (index, chunk) in input.as_bytes().chunks(4).enumerate() {
        let mut hasher = DefaultHasher::new();
        chunk.hash(&mut hasher);
        index.hash(&mut hasher);
        let value = (hasher.finish() % 10_000) as f32 / 10_000.0;
        output[index % size] += value;
    }
    output
}
pub async fn embed_texts(
    model_path: &Path,
    model_file: &str,
    texts: &[String],
    fallback_dimensions: usize,
    max_tokens: usize,
) -> AppResult<Vec<Vec<f32>>> {
    if !supports_real_embedding(model_path, model_file) {
        return Ok(texts
            .iter()
            .map(|text| deterministic_embedding(text, fallback_dimensions))
            .collect());
    }

    let model_path = model_path.to_path_buf();
    let model_file = model_file.to_string();
    let texts = texts.to_vec();
    task::spawn_blocking(move || {
        let runtime = runtime_for_model(&model_path, &model_file)?;
        infer_embeddings(&runtime, &texts, max_tokens)
    })
        .await
        .map_err(|error| AppError::Internal(format!("embedding worker failed: {error}")))?
}

pub async fn rerank_documents(
    metadata: &ModelMetadata,
    model_path: &Path,
    model_file: &str,
    query: &str,
    documents: &[String],
    max_tokens: usize,
) -> AppResult<Vec<f32>> {
    if !supports_real_rerank(model_path, model_file) {
        return Err(AppError::BadRequest(format!(
            "model bundle at {} is not a valid reranker bundle",
            model_path.display()
        )));
    }

    let model_path = model_path.to_path_buf();
    let model_file = model_file.to_string();
    let query = query.to_string();
    let documents = documents.to_vec();
    let metadata = metadata.clone();
    task::spawn_blocking(move || {
        let runtime = runtime_for_model(&model_path, &model_file)?;
        infer_rerank_scores(&runtime, &metadata, &query, &documents, max_tokens)
    })
    .await
    .map_err(|error| AppError::Internal(format!("reranking worker failed: {error}")))?
}

fn supports_real_embedding(model_path: &Path, model_file: &str) -> bool {
    model_path.join("tokenizer.json").exists()
        && model_path.join(model_file).exists()
        && model_file.ends_with(".onnx")
}

fn supports_real_rerank(model_path: &Path, model_file: &str) -> bool {
    model_path.join("tokenizer.json").exists()
        && model_path.join(model_file).exists()
        && model_file.ends_with(".onnx")
}

fn runtime_for_model(model_path: &Path, model_file: &str) -> AppResult<Arc<RustModelRuntime>> {
    let cache_key = runtime_cache_key(model_path, model_file);

    if let Some(runtime) = runtime_cache().lock().unwrap().get(&cache_key).cloned() {
        return Ok(runtime);
    }

    let runtime = Arc::new(load_runtime(model_path, model_file)?);
    let mut cache = runtime_cache().lock().unwrap();
    Ok(cache.entry(cache_key).or_insert_with(|| Arc::clone(&runtime)).clone())
}

fn runtime_cache() -> &'static Mutex<HashMap<String, Arc<RustModelRuntime>>> {
    RUST_RUNTIME_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn runtime_cache_key(model_path: &Path, model_file: &str) -> String {
    format!("{}::{}", model_path.display(), model_file)
}

fn load_runtime(model_path: &Path, model_file: &str) -> AppResult<RustModelRuntime> {
    let tokenizer = Arc::new(load_tokenizer(model_path)?);
    let model_file_path = model_path.join(model_file);
    let builder = Session::builder()
        .map_err(|error| AppError::Internal(format!("failed to create ONNX session builder: {error}")))?;
    let builder = if model_file.contains("fp16") {
        builder
            .with_optimization_level(GraphOptimizationLevel::Disable)
            .map_err(|error| {
                AppError::Internal(format!("failed to configure ONNX session builder: {error}"))
            })?
    } else {
        builder
    };
    let session = builder
        .commit_from_file(&model_file_path)
        .map_err(|error| {
            AppError::Internal(format!(
                "failed to load ONNX model {}: {error}",
                model_file_path.display()
            ))
        })?;

    let input_names = session
        .inputs()
        .iter()
        .map(|input| input.name().to_string())
        .collect();

    Ok(RustModelRuntime {
        tokenizer,
        session: Mutex::new(session),
        input_names,
    })
}

fn load_tokenizer(model_path: &Path) -> AppResult<Tokenizer> {
    let tokenizer_path = model_path.join("tokenizer.json");
    if !tokenizer_path.exists() {
        return Err(AppError::BadRequest(format!(
            "model bundle at {} does not include tokenizer.json",
            model_path.display()
        )));
    }

    Tokenizer::from_file(&tokenizer_path).map_err(|error| {
        AppError::Internal(format!(
            "failed to load tokenizer {}: {error}",
            tokenizer_path.display()
        ))
    })
}

fn infer_embeddings(
    runtime: &RustModelRuntime,
    texts: &[String],
    max_tokens: usize,
) -> AppResult<Vec<Vec<f32>>> {
    let batch = prepare_batch(&runtime.tokenizer, texts, max_tokens)?;
    let inputs = build_inputs(&runtime.input_names, &batch)?;
    let mut session = runtime.session.lock().unwrap();
    let outputs = session
        .run(inputs)
        .map_err(|error| AppError::Internal(format!("embedding inference failed: {error}")))?;

    let output = outputs
        .into_iter()
        .next()
        .ok_or_else(|| AppError::Internal("embedding model returned no outputs".to_string()))?;
    let (shape, values) = output
        .1
        .try_extract_tensor::<f32>()
        .map_err(|error| AppError::Internal(format!("failed to read embedding output tensor: {error}")))?;
    let dims: Vec<usize> = shape.iter().map(|dim| *dim as usize).collect();

    match dims.as_slice() {
        [batch_size, hidden_size] => {
            if *batch_size != batch.batch_size {
                return Err(AppError::Internal(format!(
                    "embedding model returned {} rows for {} inputs",
                    batch_size,
                    batch.batch_size
                )));
            }
            Ok(values.chunks(*hidden_size).map(|row| row.to_vec()).collect())
        }
        [batch_size, seq_len, hidden_size] => {
            if *batch_size != batch.batch_size {
                return Err(AppError::Internal(format!(
                    "embedding model returned {} rows for {} inputs",
                    batch_size,
                    batch.batch_size
                )));
            }
            Ok(pool_embeddings(values, *batch_size, *seq_len, *hidden_size, &batch.attention_mask))
        }
        _ => Err(AppError::Internal(format!("unexpected embedding output shape: {:?}", dims))),
    }
}

fn infer_rerank_scores(
    runtime: &RustModelRuntime,
    metadata: &ModelMetadata,
    query: &str,
    documents: &[String],
    max_tokens: usize,
) -> AppResult<Vec<f32>> {
    let batch = prepare_pair_batch(&runtime.tokenizer, metadata, query, documents, max_tokens)?;
    let inputs = build_inputs(&runtime.input_names, &batch)?;
    let mut session = runtime.session.lock().unwrap();
    let outputs = session
        .run(inputs)
        .map_err(|error| AppError::Internal(format!("reranking inference failed: {error}")))?;

    let output = outputs
        .into_iter()
        .next()
        .ok_or_else(|| AppError::Internal("reranking model returned no outputs".to_string()))?;
    let (shape, values): (_, Vec<f32>) = match output.1.try_extract_tensor::<f32>() {
        Ok((shape, values)) => (shape, values.to_vec()),
        Err(f32_error) => match output.1.try_extract_tensor::<f16>() {
            Ok((shape, values)) => {
                let values = values.iter().map(|value| value.to_f32()).collect();
                (shape, values)
            }
            Err(f16_error) => {
                return Err(AppError::Internal(format!(
                    "failed to read rerank output tensor as f32 or f16: {f32_error}; {f16_error}"
                )));
            }
        },
    };
    let dims: Vec<usize> = shape.iter().map(|dim| *dim as usize).collect();

    if is_qwen3_reranker(metadata) {
        return extract_qwen3_rerank_scores(&runtime.tokenizer, &batch, &values, &dims);
    }

    match dims.as_slice() {
        [batch_size] => {
            if *batch_size != documents.len() {
                return Err(AppError::Internal(format!(
                    "reranking model returned {} scores for {} documents",
                    batch_size,
                    documents.len()
                )));
            }
            Ok(values.to_vec())
        }
        [batch_size, 1] => {
            if *batch_size != documents.len() {
                return Err(AppError::Internal(format!(
                    "reranking model returned {} scores for {} documents",
                    batch_size,
                    documents.len()
                )));
            }
            Ok(values.chunks(1).map(|row| row[0]).collect())
        }
        [batch_size, classes] if *batch_size == documents.len() => {
            let mut scores = Vec::with_capacity(*batch_size);
            for row in values.chunks(*classes) {
                let max = row.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let mut denom = 0.0f32;
                let mut last_prob = 0.0f32;
                for (index, value) in row.iter().copied().enumerate() {
                    let exp = (value - max).exp();
                    denom += exp;
                    if index + 1 == *classes {
                        last_prob = exp;
                    }
                }
                scores.push(last_prob / denom.max(1e-9));
            }
            Ok(scores)
        }
        _ => Err(AppError::Internal(format!("unexpected reranking output shape: {:?}", dims))),
    }
}

fn is_qwen3_reranker(metadata: &ModelMetadata) -> bool {
    metadata.family == "qwen3" && metadata.task == "rerank"
}

fn extract_qwen3_rerank_scores(
    tokenizer: &Tokenizer,
    batch: &PreparedBatch,
    values: &[f32],
    dims: &[usize],
) -> AppResult<Vec<f32>> {
    let [batch_size, seq_len, vocab_size] = dims else {
        return Err(AppError::Internal(format!(
            "unexpected qwen3 reranking output shape: {:?}",
            dims
        )));
    };

    if *batch_size != batch.batch_size {
        return Err(AppError::Internal(format!(
            "reranking model returned {} rows for {} documents",
            batch_size, batch.batch_size
        )));
    }

    let true_id = tokenizer
        .token_to_id("yes")
        .ok_or_else(|| AppError::Internal("qwen3 reranker tokenizer is missing token: yes".to_string()))?
        as usize;
    let false_id = tokenizer
        .token_to_id("no")
        .ok_or_else(|| AppError::Internal("qwen3 reranker tokenizer is missing token: no".to_string()))?
        as usize;

    if true_id >= *vocab_size || false_id >= *vocab_size {
        return Err(AppError::Internal(format!(
            "qwen3 reranker token ids exceed vocab size {}",
            vocab_size
        )));
    }

    let mut scores = Vec::with_capacity(*batch_size);
    for batch_index in 0..*batch_size {
        let row_offset = batch_index * batch.seq_len;
        let last_token_index = batch.attention_mask[row_offset..row_offset + batch.seq_len]
            .iter()
            .rposition(|value| *value != 0)
            .ok_or_else(|| {
                AppError::Internal(format!(
                    "qwen3 reranker received an empty sequence at index {}",
                    batch_index
                ))
            })?;
        let base = batch_index * seq_len * vocab_size + last_token_index * vocab_size;
        let false_logits = values[base + false_id];
        let true_logits = values[base + true_id];
        let max_logit = false_logits.max(true_logits);
        let false_exp = (false_logits - max_logit).exp();
        let true_exp = (true_logits - max_logit).exp();
        scores.push(true_exp / (false_exp + true_exp).max(1e-9));
    }

    Ok(scores)
}

struct PreparedBatch {
    input_ids: Vec<i64>,
    attention_mask: Vec<i64>,
    token_type_ids: Vec<i64>,
    position_ids: Vec<i64>,
    batch_size: usize,
    seq_len: usize,
}

fn prepare_batch(tokenizer: &Tokenizer, texts: &[String], max_tokens: usize) -> AppResult<PreparedBatch> {
    if texts.is_empty() {
        return Err(AppError::BadRequest("input must not be empty".to_string()));
    }

    let mut encodings = Vec::with_capacity(texts.len());
    let mut seq_len = 0usize;

    for text in texts {
        let encoding = tokenizer
            .encode(text.as_str(), true)
            .map_err(|error| AppError::BadRequest(format!("failed to tokenize input: {error}")))?;
        seq_len = seq_len.max(encoding.get_ids().len().min(max_tokens.max(1)));
        encodings.push(encoding);
    }

    let mut input_ids = Vec::with_capacity(texts.len() * seq_len);
    let mut attention_mask = Vec::with_capacity(texts.len() * seq_len);
    let mut token_type_ids = Vec::with_capacity(texts.len() * seq_len);
    let mut position_ids = Vec::with_capacity(texts.len() * seq_len);
    let pad_id = tokenizer
        .get_padding()
        .map(|padding| padding.pad_id as i64)
        .or_else(|| tokenizer.token_to_id("<pad>").map(|id| id as i64))
        .or_else(|| tokenizer.token_to_id("[PAD]").map(|id| id as i64))
        .unwrap_or(0);

    for encoding in encodings {
        let mut ids: Vec<i64> = encoding
            .get_ids()
            .iter()
            .take(max_tokens.max(1))
            .map(|id| *id as i64)
            .collect();
        let mut mask: Vec<i64> = vec![1; ids.len()];
        let mut type_ids: Vec<i64> = encoding
            .get_type_ids()
            .iter()
            .take(ids.len())
            .map(|id| *id as i64)
            .collect();

        ids.resize(seq_len, pad_id);
        mask.resize(seq_len, 0);
        type_ids.resize(seq_len, 0);

        input_ids.extend(ids);
        attention_mask.extend(mask);
        token_type_ids.extend(type_ids);
        position_ids.extend((0..seq_len).map(|position| position as i64));
    }

    Ok(PreparedBatch {
        input_ids,
        attention_mask,
        token_type_ids,
        position_ids,
        batch_size: texts.len(),
        seq_len,
    })
}

fn prepare_pair_batch(
    tokenizer: &Tokenizer,
    metadata: &ModelMetadata,
    query: &str,
    documents: &[String],
    max_tokens: usize,
) -> AppResult<PreparedBatch> {
    if documents.is_empty() {
        return Err(AppError::BadRequest("documents must not be empty".to_string()));
    }

    let mut batches = Vec::with_capacity(documents.len());
    let mut seq_len = 0usize;
    let qwen3_reranker = is_qwen3_reranker(metadata);

    for document in documents {
        let (ids, type_ids) = if qwen3_reranker {
            let ids = qwen3_rerank_input_ids(tokenizer, query, document, max_tokens)?;
            let type_ids = vec![0; ids.len()];
            (ids, type_ids)
        } else {
            let encoding = tokenizer
                .encode((query, document.as_str()), true)
                .map_err(|error| AppError::BadRequest(format!("failed to tokenize rerank input: {error}")))?
            ;
            (
                encoding
                    .get_ids()
                    .iter()
                    .take(max_tokens.max(1))
                    .map(|id| *id as i64)
                    .collect(),
                encoding
                    .get_type_ids()
                    .iter()
                    .take(max_tokens.max(1))
                    .map(|id| *id as i64)
                    .collect(),
            )
        };
        seq_len = seq_len.max(ids.len());
        batches.push((ids, type_ids));
    }

    let mut input_ids = Vec::with_capacity(documents.len() * seq_len);
    let mut attention_mask = Vec::with_capacity(documents.len() * seq_len);
    let mut token_type_ids = Vec::with_capacity(documents.len() * seq_len);
    let mut position_ids = Vec::with_capacity(documents.len() * seq_len);
    let pad_id = tokenizer
        .get_padding()
        .map(|padding| padding.pad_id as i64)
        .or_else(|| tokenizer.token_to_id("<pad>").map(|id| id as i64))
        .or_else(|| tokenizer.token_to_id("[PAD]").map(|id| id as i64))
        .unwrap_or(0);

    for (mut ids, mut type_ids) in batches {
        let mut mask: Vec<i64> = vec![1; ids.len()];

        ids.resize(seq_len, pad_id);
        mask.resize(seq_len, 0);
        type_ids.resize(seq_len, 0);

        input_ids.extend(ids);
        attention_mask.extend(mask);
        token_type_ids.extend(type_ids);
        position_ids.extend((0..seq_len).map(|position| position as i64));
    }

    Ok(PreparedBatch {
        input_ids,
        attention_mask,
        token_type_ids,
        position_ids,
        batch_size: documents.len(),
        seq_len,
    })
}

pub fn qwen3_rerank_prompt(query: &str, document: &str) -> String {
    format!(
        "<Instruct>: {QWEN3_RERANK_INSTRUCTION}\n<Query>: {query}\n<Document>: {document}"
    )
}

fn qwen3_rerank_input_ids(
    tokenizer: &Tokenizer,
    query: &str,
    document: &str,
    max_tokens: usize,
) -> AppResult<Vec<i64>> {
    let limit = max_tokens.max(1);
    let prefix_ids = tokenizer
        .encode(QWEN3_RERANK_PREFIX, false)
        .map_err(|error| AppError::BadRequest(format!("failed to tokenize rerank prefix: {error}")))?
        .get_ids()
        .iter()
        .map(|id| *id as i64)
        .collect::<Vec<_>>();
    let content_ids = tokenizer
        .encode(qwen3_rerank_prompt(query, document).as_str(), false)
        .map_err(|error| AppError::BadRequest(format!("failed to tokenize rerank input: {error}")))?
        .get_ids()
        .iter()
        .map(|id| *id as i64)
        .collect::<Vec<_>>();
    let suffix_ids = qwen3_rerank_suffix_ids(tokenizer)?;

    let mut ids = Vec::with_capacity(limit);
    ids.extend(prefix_ids.into_iter().take(limit));

    let suffix_keep = suffix_ids.len().min(limit.saturating_sub(ids.len()));
    let content_keep = limit.saturating_sub(ids.len() + suffix_keep);
    ids.extend(content_ids.into_iter().take(content_keep));
    ids.extend(suffix_ids.into_iter().take(limit.saturating_sub(ids.len())));

    Ok(ids)
}

const QWEN3_RERANK_PREFIX: &str = "<|im_start|>system\nJudge whether the Document meets the requirements based on the Query and the Instruct provided. Note that the answer can only be \"yes\" or \"no\".\n<|im_end|>\n<|im_start|>user\n";
const QWEN3_RERANK_SUFFIX: &str = "<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n";
const QWEN3_RERANK_INSTRUCTION: &str = "Given a web search query, retrieve relevant passages that answer the query";

fn qwen3_rerank_suffix_ids(tokenizer: &Tokenizer) -> AppResult<Vec<i64>> {
    tokenizer
        .encode(QWEN3_RERANK_SUFFIX, false)
        .map_err(|error| AppError::BadRequest(format!("failed to tokenize rerank suffix: {error}")))
        .map(|encoding| encoding.get_ids().iter().map(|id| *id as i64).collect())
}

fn build_inputs(input_names: &[String], batch: &PreparedBatch) -> AppResult<HashMap<String, DynValue>> {
    let mut inputs = HashMap::new();

    for name in input_names {
        let key = normalize_input_name(name);
        let tensor: DynValue = match key.as_str() {
            "input_ids" => Tensor::from_array((
                vec![batch.batch_size, batch.seq_len],
                batch.input_ids.clone().into_boxed_slice(),
            ))
            .map(Into::into)
            .map_err(|error| AppError::Internal(format!("failed to build tensor for {name}: {error}")))?,
            "attention_mask" => Tensor::from_array((
                vec![batch.batch_size, batch.seq_len],
                batch.attention_mask.clone().into_boxed_slice(),
            ))
            .map(Into::into)
            .map_err(|error| AppError::Internal(format!("failed to build tensor for {name}: {error}")))?,
            "token_type_ids" | "segment_ids" => Tensor::from_array((
                vec![batch.batch_size, batch.seq_len],
                batch.token_type_ids.clone().into_boxed_slice(),
            ))
            .map(Into::into)
            .map_err(|error| AppError::Internal(format!("failed to build tensor for {name}: {error}")))?,
            "position_ids" => Tensor::from_array((
                vec![batch.batch_size, batch.seq_len],
                batch.position_ids.clone().into_boxed_slice(),
            ))
            .map(Into::into)
            .map_err(|error| AppError::Internal(format!("failed to build tensor for {name}: {error}")))?,
            _ if key.starts_with("past_key_values.")
                && (key.ends_with(".key") || key.ends_with(".value")) =>
            Tensor::<f32>::new(&Allocator::default(), [batch.batch_size, 8, 0, 128])
            .map(Into::into)
            .map_err(|error| AppError::Internal(format!("failed to build tensor for {name}: {error}")))?,
            _ => {
                return Err(AppError::Internal(format!("unsupported ONNX input name: {name}")));
            }
        };

        inputs.insert(name.clone(), tensor);
    }

    Ok(inputs)
}

fn normalize_input_name(name: &str) -> String {
    let lowered = name.to_ascii_lowercase();
    if lowered.contains("input_ids") || lowered == "input" {
        "input_ids".to_string()
    } else if lowered.contains("attention_mask") || lowered.contains("mask") {
        "attention_mask".to_string()
    } else if lowered.contains("token_type") || lowered.contains("segment") {
        "token_type_ids".to_string()
    } else if lowered.contains("position") {
        "position_ids".to_string()
    } else {
        lowered
    }
}

fn pool_embeddings(
    values: &[f32],
    batch_size: usize,
    seq_len: usize,
    hidden_size: usize,
    attention_mask: &[i64],
) -> Vec<Vec<f32>> {
    let mut pooled = Vec::with_capacity(batch_size);
    for batch_index in 0..batch_size {
        let mut row = vec![0.0f32; hidden_size];
        let mut weight_sum = 0.0f32;
        for token_index in 0..seq_len {
            let weight = attention_mask[batch_index * seq_len + token_index] as f32;
            if weight == 0.0 {
                continue;
            }
            weight_sum += weight;
            let base = batch_index * seq_len * hidden_size + token_index * hidden_size;
            for hidden_index in 0..hidden_size {
                row[hidden_index] += values[base + hidden_index] * weight;
            }
        }

        let denom = weight_sum.max(1e-9);
        for value in &mut row {
            *value /= denom;
        }
        pooled.push(row);
    }

    pooled
}

pub fn token_count(text: &str) -> usize {
    text.split_whitespace().count().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qwen3_reranker_prompt_wraps_query_and_document() {
        let prompt = qwen3_rerank_prompt("what is rust", "a systems language");

        assert!(prompt.contains("<Instruct>: Given a web search query, retrieve relevant passages that answer the query"));
        assert!(prompt.contains("<Query>: what is rust"));
        assert!(prompt.contains("<Document>: a systems language"));
        assert!(!prompt.contains("<|im_start|>system"));
        assert!(!prompt.contains("<|im_end|>\n<|im_start|>assistant"));
    }

    #[test]
    fn qwen3_reranker_detection_uses_family_and_task() {
        let metadata = ModelMetadata {
            name: "Qwen3-Reranker-0.6B-ONNX".to_string(),
            directory: "Qwen3-Reranker-0.6B-ONNX".to_string(),
            family: "qwen3".to_string(),
            task: "rerank".to_string(),
            dimensions: 1024,
            max_tokens: 32000,
            supported_types: vec!["rerank".to_string(), "token_count".to_string()],
            default_model_file: "model.onnx".to_string(),
            files: vec!["model.onnx".to_string()],
            tokenizer_class: "Qwen2Tokenizer".to_string(),
            source_model: "Qwen/Qwen3-Reranker-0.6B".to_string(),
            notes: "test".to_string(),
        };

        assert!(is_qwen3_reranker(&metadata));
    }
}
