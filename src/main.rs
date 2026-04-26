use dotenv::dotenv;
use embedding_api_server::{config::AppConfig, registry::ModelRegistry, routes};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let config = AppConfig::from_env();
    let registry = ModelRegistry::discover(&config).await?;
    registry.spawn_reaper();
    let bind_addr = config.bind_addr;
    let app = routes::router(registry, config);

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    tracing::info!(addr = %listener.local_addr()?, "starting server");
    axum::serve(listener, app).await?;
    Ok(())
}
