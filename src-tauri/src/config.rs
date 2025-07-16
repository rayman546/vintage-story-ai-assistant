use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub ollama: OllamaConfig,
    pub wiki: WikiConfig,
    pub embedding: EmbeddingConfig,
    pub chat: ChatConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub host: String,
    pub port: u16,
    pub model_name: String,
    pub installation_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiConfig {
    pub base_url: String,
    pub update_interval_hours: u64,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    pub model_name: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    pub max_context_chunks: usize,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ollama: OllamaConfig::default(),
            wiki: WikiConfig::default(),
            embedding: EmbeddingConfig::default(),
            chat: ChatConfig::default(),
        }
    }
}


impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 11434,
            model_name: "phi3:mini".to_string(),
            installation_path: None,
        }
    }
}

impl Default for WikiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://wiki.vintagestory.at".to_string(),
            update_interval_hours: 24,
            last_update: None,
        }
    }
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model_name: "nomic-embed-text".to_string(),
            chunk_size: 512,
            chunk_overlap: 50,
            batch_size: 10,
        }
    }
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            max_context_chunks: 5,
            temperature: 0.7,
            max_tokens: 1024,
        }
    }
}

impl AppConfig {
    pub fn load() -> crate::errors::AppResult<Self> {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| crate::errors::AppError::ConfigError(
                    format!("Failed to read config file: {}", e)
                ))?;
            
            let config: AppConfig = serde_json::from_str(&content)
                .map_err(|e| crate::errors::AppError::ConfigError(
                    format!("Failed to parse config file: {}", e)
                ))?;
            
            Ok(config)
        } else {
            // Create default config and save it
            let default_config = Self::default();
            
            // Create config directory if it doesn't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| crate::errors::AppError::ConfigError(
                        format!("Failed to create config directory: {}", e)
                    ))?;
            }
            
            default_config.save()?;
            Ok(default_config)
        }
    }
    
    pub fn save(&self) -> crate::errors::AppResult<()> {
        let config_path = Self::get_config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| crate::errors::AppError::ConfigError(
                    format!("Failed to create config directory: {}", e)
                ))?;
        }
        
        // Serialize config with pretty formatting
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| crate::errors::AppError::ConfigError(
                format!("Failed to serialize config: {}", e)
            ))?;
        
        // Atomic write: write to temporary file first, then rename
        let temp_path = config_path.with_extension("tmp");
        
        fs::write(&temp_path, content)
            .map_err(|e| crate::errors::AppError::ConfigError(
                format!("Failed to write temporary config file: {}", e)
            ))?;
        
        // Atomic rename to final location
        fs::rename(&temp_path, &config_path)
            .map_err(|e| crate::errors::AppError::ConfigError(
                format!("Failed to finalize config file: {}", e)
            ))?;
        
        Ok(())
    }
    
    pub fn get_data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vintage-story-ai-assistant")
    }
    
    fn get_config_path() -> PathBuf {
        Self::get_data_dir().join("config.json")
    }
}
