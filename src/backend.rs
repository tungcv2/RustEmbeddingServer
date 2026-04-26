use crate::{
    error::{AppError, AppResult},
    models::ModelMetadata,
};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{path::Path, process::Command};
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

#[derive(Debug, Deserialize)]
struct PythonEmbeddingResponse {
    embeddings: Vec<Vec<f32>>,
}

const PYTHON_EMBEDDING_SCRIPT: &str = r#"
import json
import os

payload = json.loads(os.environ['EMBEDDING_REQUEST_JSON'])
model_path = payload['model_path']
model_file = payload.get('model_file')
texts = payload['texts']
max_tokens = int(payload['max_tokens'])

def pick_onnx_file(path, preferred_file):
    if preferred_file:
        candidate = os.path.join(path, preferred_file)
        if os.path.exists(candidate):
            return candidate

    for root, _dirs, files in os.walk(path):
        for file_name in files:
            if file_name.endswith('.onnx'):
                return os.path.join(root, file_name)
    return None

onnx_file = pick_onnx_file(model_path, model_file)
if onnx_file is None:
    raise RuntimeError(f'no ONNX file found in {model_path}')

from transformers import AutoTokenizer
import numpy as np
import onnxruntime as ort

tokenizer = AutoTokenizer.from_pretrained(
    model_path,
    trust_remote_code=True,
    local_files_only=True,
)

session = ort.InferenceSession(onnx_file, providers=['CPUExecutionProvider'])
batch = tokenizer(
    texts,
    padding=True,
    truncation=True,
    max_length=max_tokens,
    return_tensors='np',
)

session_inputs = {}
for input_info in session.get_inputs():
    if input_info.name in batch:
        session_inputs[input_info.name] = batch[input_info.name]
    elif input_info.name == 'token_type_ids':
        session_inputs[input_info.name] = np.zeros_like(batch['input_ids'])

outputs = session.run(None, session_inputs)
embeddings = np.asarray(outputs[0])
if embeddings.ndim == 3:
    attention_mask = batch['attention_mask'][..., None].astype(np.float32)
    pooled = (embeddings * attention_mask).sum(axis=1) / np.clip(attention_mask.sum(axis=1), 1e-9, None)
    embeddings = pooled
elif embeddings.ndim != 2:
    raise RuntimeError(f'unexpected ONNX output shape: {embeddings.shape}')

embeddings = embeddings.tolist()

print(json.dumps({'embeddings': embeddings}, ensure_ascii=False))
"#;

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
    task::spawn_blocking(move || run_python_embedding(&model_path, &model_file, &texts, max_tokens))
        .await
        .map_err(|error| AppError::Internal(format!("embedding worker failed: {error}")))?
}

fn supports_real_embedding(model_path: &Path, model_file: &str) -> bool {
    let has_manifest = ["config.json", "tokenizer_config.json"]
        .iter()
        .any(|name| model_path.join(name).exists());
    let has_tokenizer = [
        "tokenizer.json",
        "tokenizer.model",
        "vocab.txt",
        "bpe.codes",
    ]
    .iter()
    .any(|name| model_path.join(name).exists());
    let has_model = model_path.join(model_file).exists() && model_file.ends_with(".onnx");

    has_manifest && has_tokenizer && has_model
}

fn run_python_embedding(
    model_path: &Path,
    model_file: &str,
    texts: &[String],
    max_tokens: usize,
) -> AppResult<Vec<Vec<f32>>> {
    let request = serde_json::json!({
        "model_path": model_path.to_string_lossy(),
        "model_file": model_file,
        "texts": texts,
        "max_tokens": max_tokens,
    })
    .to_string();

    let mut last_error = None;
    for interpreter in ["python", "python3"] {
        let output = Command::new(interpreter)
            .arg("-c")
            .arg(PYTHON_EMBEDDING_SCRIPT)
            .env("EMBEDDING_REQUEST_JSON", &request)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let parsed: PythonEmbeddingResponse = serde_json::from_slice(&output.stdout)?;
                if parsed.embeddings.len() != texts.len() {
                    return Err(AppError::Internal(format!(
                        "embedding model returned {} vectors for {} inputs",
                        parsed.embeddings.len(),
                        texts.len()
                    )));
                }
                return Ok(parsed.embeddings);
            }
            Ok(output) => {
                last_error = Some(String::from_utf8_lossy(&output.stderr).trim().to_string());
            }
            Err(error) => {
                last_error = Some(error.to_string());
            }
        }
    }

    Err(AppError::Internal(format!(
        "real embedding inference failed: {}",
        last_error.unwrap_or_else(|| "no Python interpreter found".to_string())
    )))
}

pub fn token_count(text: &str) -> usize {
    text.split_whitespace().count().max(1)
}
