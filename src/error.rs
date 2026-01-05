use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProssimoError {
    #[error("Configuration file missing: {0}")]
    ConfigMissing(String),
    #[error("Invalid configuration: {0}")]
    ConfigInvalid(String),
    #[error("Server failed to start: {0}")]
    ServerStart(String),
    #[error("IO error")]
    Io(std::io::Error),
    #[error("Network error: {0}")]  
    Network(String),
    #[error("Generic error: {0}")]
    Generic(String),
    #[error("Other error")]
    Other { source: anyhow::Error },
    #[error("Invalid Command Line Argument: {0}")]
    CLIArgumentError(String),
}

impl From<std::io::Error> for ProssimoError {
    fn from(err: std::io::Error) -> Self {
        ProssimoError::Io(err)
    }
}

impl From<anyhow::Error> for ProssimoError {
    fn from(err: anyhow::Error) -> Self {
        ProssimoError::Other { source: err }
    }
}




