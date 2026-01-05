use tracing_subscriber::EnvFilter;

pub fn init(config: &crate::config::Config) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(&config.logging.level))
        .init();
    Ok(())
}

pub fn info(message: &str) {
    tracing::info!("{}",message);
}

pub fn error(message: &str) {
    tracing::error!("{}",message);
}   

pub fn debug(message: &str) {
    tracing::debug!("{}",message);
}

pub fn warn(message: &str) {
    tracing::warn!("{}",message);
}
pub fn trace(message: &str) {
    tracing::trace!("{}",message);
}
