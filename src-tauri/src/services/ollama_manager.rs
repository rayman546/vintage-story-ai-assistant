use crate::config::OllamaConfig;
use crate::errors::{AppError, AppResult};
use log::{info, warn, error};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use tokio::time::{sleep, Duration};
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDetails {
    pub parameter_size: String,
    pub quantization_level: String,
    pub family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaStatus {
    pub is_running: bool,
    pub is_installed: bool,
    pub version: Option<String>,
    pub models: Vec<ModelInfo>,
}

pub struct OllamaManager {
    config: OllamaConfig,
    client: Client,
    process: Option<Child>,
}

impl OllamaManager {
    pub async fn new() -> Self {
        let config = OllamaConfig::default();
        let client = Client::new();
        
        Self {
            config,
            client,
            process: None,
        }
    }
    
    pub async fn get_status(&self) -> AppResult<OllamaStatus> {
        let is_installed = self.check_installation().await;
        let is_running = self.check_health().await.is_ok();
        let version = if is_running {
            self.get_version().await.ok()
        } else {
            None
        };
        let models = if is_running {
            self.list_models().await.unwrap_or_default()
        } else {
            Vec::new()
        };
        
        Ok(OllamaStatus {
            is_running,
            is_installed,
            version,
            models,
        })
    }

    async fn check_installation(&self) -> bool {
        // Check if ollama executable exists
        Command::new("ollama")
            .arg("--version")
            .output()
            .is_ok()
    }
    
    pub async fn check_health(&self) -> AppResult<()> {
        let url = format!("http://{}:{}/api/tags", self.config.host, self.config.port);
        
        match self.client.get(&url).send().await {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(AppError::OllamaError(
                format!("Ollama health check failed with status: {}", response.status())
            )),
            Err(e) => Err(AppError::OllamaError(
                format!("Failed to connect to Ollama: {}", e)
            )),
        }
    }
    
    pub async fn start_service(&mut self) -> AppResult<()> {
        if self.check_health().await.is_ok() {
            info!("Ollama is already running");
            return Ok(());
        }
        
        info!("Starting Ollama service...");
        
        let mut cmd = Command::new("ollama");
        cmd.arg("serve")
           .stdout(Stdio::null())
           .stderr(Stdio::null());
        
        match cmd.spawn() {
            Ok(child) => {
                self.process = Some(child);
                
                // Wait for service to be ready
                for _ in 0..30 {
                    sleep(Duration::from_secs(1)).await;
                    if self.check_health().await.is_ok() {
                        info!("Ollama service started successfully");
                        return Ok(());
                    }
                }
                
                Err(AppError::OllamaError("Ollama service failed to start within timeout".to_string()))
            }
            Err(e) => Err(AppError::OllamaError(format!("Failed to start Ollama: {}", e))),
        }
    }
    
    async fn get_version(&self) -> AppResult<String> {
        let url = format!("http://{}:{}/api/version", self.config.host, self.config.port);
        
        let response = self.client.get(&url).send().await?;
        let version_info: serde_json::Value = response.json().await?;
        
        Ok(version_info["version"]
            .as_str()
            .unwrap_or("unknown")
            .to_string())
    }
    
    pub async fn list_models(&self) -> AppResult<Vec<ModelInfo>> {
        let url = format!("http://{}:{}/api/tags", self.config.host, self.config.port);
        
        let response = self.client.get(&url).send().await?;
        let models_response: serde_json::Value = response.json().await?;
        
        let models = models_response["models"]
            .as_array()
            .ok_or_else(|| AppError::OllamaError("Invalid models response".to_string()))?;
        
        let mut model_list = Vec::new();
        for model in models {
            if let Ok(model_info) = serde_json::from_value::<ModelInfo>(model.clone()) {
                model_list.push(model_info);
            }
        }
        
        Ok(model_list)
    }
    
    pub async fn download_model(&self, model_name: &str) -> AppResult<()> {
        info!("Downloading model: {}", model_name);
        
        let url = format!("http://{}:{}/api/pull", self.config.host, self.config.port);
        let payload = serde_json::json!({
            "name": model_name
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        if response.status().is_success() {
            info!("Model {} downloaded successfully", model_name);
            Ok(())
        } else {
            Err(AppError::OllamaError(
                format!("Failed to download model {}: {}", model_name, response.status())
            ))
        }
    }
    
    pub async fn generate_response(&self, prompt: &str) -> AppResult<String> {
        let url = format!("http://{}:{}/api/generate", self.config.host, self.config.port);
        let payload = serde_json::json!({
            "model": self.config.model_name,
            "prompt": prompt,
            "stream": false
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        let result: serde_json::Value = response.json().await?;
        
        Ok(result["response"]
            .as_str()
            .unwrap_or("No response generated")
            .to_string())
    }
    
    pub async fn ensure_available(&mut self) -> AppResult<()> {
        info!("Ensuring Ollama is available");
        
        // Check if Ollama is already running
        if self.check_health().await.is_ok() {
            info!("Ollama is already running");
            return Ok(());
        }
        
        // Check if Ollama is installed
        if !self.check_installation().await {
            info!("Ollama not found, attempting to install");
            self.install_ollama().await?;
        }
        
        // Start Ollama service
        self.start_service().await?;
        
        // Ensure we have at least one model
        self.ensure_model_available().await?;
        
        Ok(())
    }
    
    async fn install_ollama(&self) -> AppResult<()> {
        info!("Installing Ollama for platform: {}", std::env::consts::OS);
        
        match std::env::consts::OS {
            "windows" => self.install_windows().await,
            "macos" => self.install_macos().await,
            "linux" => self.install_linux().await,
            _ => Err(AppError::OllamaError(
                format!("Unsupported platform: {}", std::env::consts::OS)
            )),
        }
    }    
    async fn install_windows(&self) -> AppResult<()> {
        use std::process::Command;
        use std::env;
        
        info!("Installing Ollama on Windows");
        
        // Download URL for Windows installer
        let download_url = "https://ollama.ai/download/OllamaSetup.exe";
        let temp_dir = env::temp_dir();
        let installer_path = temp_dir.join("OllamaSetup.exe");
        
        // Download the installer
        info!("Downloading Ollama installer from: {}", download_url);
        let response = self.client
            .get(download_url)
            .send()
            .await
            .map_err(|e| AppError::OllamaError(format!("Failed to download installer: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(AppError::OllamaError(
                format!("Failed to download installer: HTTP {}", response.status())
            ));
        }
        
        // Save installer to temp file
        let installer_bytes = response.bytes().await
            .map_err(|e| AppError::OllamaError(format!("Failed to read installer: {}", e)))?;
        
        std::fs::write(&installer_path, installer_bytes)
            .map_err(|e| AppError::OllamaError(format!("Failed to save installer: {}", e)))?;
        
        info!("Running Ollama installer");
        
        // Run the installer silently
        let output = Command::new(&installer_path)
            .args(&["/S"]) // Silent install
            .output()
            .map_err(|e| AppError::OllamaError(format!("Failed to run installer: {}", e)))?;
        
        if !output.status.success() {
            return Err(AppError::OllamaError(
                "Ollama installation failed".to_string()
            ));
        }
        
        // Clean up installer
        let _ = std::fs::remove_file(&installer_path);
        
        info!("Ollama installed successfully on Windows");
        
        // Give it a moment to finish installation
        sleep(Duration::from_secs(3)).await;
        
        Ok(())
    }    
    async fn install_macos(&self) -> AppResult<()> {
        use std::process::Command;
        
        info!("Installing Ollama on macOS");
        
        // Use curl to download and run the install script
        let output = Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://ollama.ai/install.sh | sh")
            .output()
            .map_err(|e| AppError::OllamaError(format!("Failed to run install script: {}", e)))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::OllamaError(
                format!("Installation failed: {}", error_msg)
            ));
        }
        
        info!("Ollama installed successfully on macOS");
        sleep(Duration::from_secs(2)).await;
        
        Ok(())
    }
    
    async fn install_linux(&self) -> AppResult<()> {
        use std::process::Command;
        
        info!("Installing Ollama on Linux");
        
        // Use the official install script
        let output = Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://ollama.ai/install.sh | sh")
            .output()
            .map_err(|e| AppError::OllamaError(format!("Failed to run install script: {}", e)))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::OllamaError(
                format!("Installation failed: {}", error_msg)
            ));
        }
        
        info!("Ollama installed successfully on Linux");
        sleep(Duration::from_secs(2)).await;
        
        Ok(())
    }    
    async fn ensure_model_available(&self) -> AppResult<()> {
        info!("Checking for available models");
        
        let models = self.list_models().await?;
        
        // Check if our configured model is available
        let model_available = models.iter()
            .any(|m| m.name.starts_with(&self.config.model_name));
        
        if !model_available {
            info!("Model {} not found, downloading...", self.config.model_name);
            self.download_model(&self.config.model_name).await?;
        } else {
            info!("Model {} is available", self.config.model_name);
        }
        
        Ok(())
    }
    
    pub async fn download_model_with_progress<F>(&self, model_name: &str, progress_callback: F) -> AppResult<()>
    where
        F: Fn(f32, String) + Send + 'static,
    {
        info!("Downloading model with progress: {}", model_name);
        
        let url = format!("http://{}:{}/api/pull", self.config.host, self.config.port);
        let payload = serde_json::json!({
            "name": model_name,
            "stream": true
        });
        
        let mut response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        // Process streaming response
        while let Some(chunk_bytes) = response.chunk().await? {
            if let Ok(text) = std::str::from_utf8(&chunk_bytes) {
                // Parse each line as JSON
                for line in text.lines() {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                        if let Some(status) = json["status"].as_str() {
                            let progress = json["completed"].as_u64().unwrap_or(0) as f32 
                                / json["total"].as_u64().unwrap_or(100) as f32;
                            progress_callback(progress, status.to_string());
                        }
                    }
                }
            }
        }
        
        info!("Model {} downloaded successfully", model_name);
        Ok(())
    }
}

#[cfg(test)]
#[path = "ollama_manager_test.rs"]
mod tests;