use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Ollama service error: {0}")]
    OllamaError(String),
    
    #[error("Wiki service error: {0}")]
    WikiError(String),
    
    #[error("Embedding service error: {0}")]
    EmbeddingError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

// Convert AppError to Tauri's Result type
impl From<AppError> for tauri::Error {
    fn from(err: AppError) -> Self {
        tauri::Error::Anyhow(anyhow::Error::new(err))
    }
}

pub type AppResult<T> = Result<T, AppError>;
