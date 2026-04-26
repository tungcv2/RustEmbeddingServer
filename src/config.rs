use std::{env, net::SocketAddr, path::PathBuf, time::Duration};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub models_dir: PathBuf,
    pub default_model: Option<String>,
    pub model_ttl: Duration,
    pub bind_addr: SocketAddr,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let models_dir = env::var("MODELS_DIR").unwrap_or_else(|_| "AI_Models".to_string());
        let default_model = env::var("DEFAULT_MODEL")
            .ok()
            .filter(|value| !value.is_empty());
        let model_ttl = env::var("MODEL_TTL_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(2 * 60 * 60));
        let bind_addr = env::var("BIND_ADDR")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| "127.0.0.1:8000".parse().expect("valid default bind addr"));

        Self {
            models_dir: PathBuf::from(models_dir),
            default_model,
            model_ttl,
            bind_addr,
        }
    }
}
