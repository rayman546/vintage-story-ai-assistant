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

impl Drop for OllamaManager {
    fn drop(&mut self) {
        if let Some(mut child) = self.process.take() {
            info!("Cleaning up Ollama process on drop");
            match child.kill() {
                Ok(_) => {
                    info!("Successfully terminated Ollama process");
                    // Wait for the process to fully terminate
                    match child.wait() {
                        Ok(status) => info!("Ollama process exited with status: {}", status),
                        Err(e) => warn!("Error waiting for Ollama process to exit: {}", e),
                    }
                }
                Err(e) => error!("Failed to kill Ollama process: {}", e),
            }
        }
    }
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
    
    pub fn set_model(&mut self, model_name: String) {
        info!("Switching to model: {}", model_name);
        self.config.model_name = model_name;
    }
    
    pub async fn generate_response(&self, prompt: &str) -> AppResult<String> {
        info!("Generating response with model: {}", self.config.model_name);
        
        let url = format!("http://{}:{}/api/generate", self.config.host, self.config.port);
        let payload = serde_json::json!({
            "model": self.config.model_name,
            "prompt": prompt,
            "stream": false
        });
        
        info!("Sending request to Ollama: {}", url);
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .timeout(Duration::from_secs(60)) // Add timeout
            .send()
            .await
            .map_err(|e| AppError::OllamaError(format!("Failed to send request to Ollama: {}", e)))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::OllamaError(format!("Ollama API error ({}): {}", status, error_text)));
        }
        
        let result: serde_json::Value = response.json().await
            .map_err(|e| AppError::OllamaError(format!("Failed to parse Ollama response: {}", e)))?;
        
        // Check for error in response
        if let Some(error) = result["error"].as_str() {
            return Err(AppError::OllamaError(format!("Ollama returned error: {}", error)));
        }
        
        let response_text = result["response"]
            .as_str()
            .unwrap_or("No response generated")
            .to_string();
        
        if response_text.is_empty() || response_text == "No response generated" {
            warn!("Empty or default response from Ollama. Full response: {:?}", result);
            return Err(AppError::OllamaError("Ollama returned empty response".to_string()));
        }
        
        info!("Successfully generated response ({} chars)", response_text.len());
        Ok(response_text)
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
    
    pub fn shutdown(&mut self) -> AppResult<()> {
        if let Some(mut child) = self.process.take() {
            info!("Shutting down Ollama process");
            match child.kill() {
                Ok(_) => {
                    info!("Successfully terminated Ollama process");
                    // Wait for the process to fully terminate with timeout
                    match child.wait() {
                        Ok(status) => {
                            info!("Ollama process exited with status: {}", status);
                            Ok(())
                        }
                        Err(e) => {
                            warn!("Error waiting for Ollama process to exit: {}", e);
                            Err(AppError::OllamaError(format!("Failed to wait for process termination: {}", e)))
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to kill Ollama process: {}", e);
                    Err(AppError::OllamaError(format!("Failed to terminate process: {}", e)))
                }
            }
        } else {
            info!("No Ollama process to shutdown");
            Ok(())
        }
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
        
        // Retry mechanism for downloads
        const MAX_RETRIES: usize = 3;
        let mut last_error = None;
        
        for attempt in 1..=MAX_RETRIES {
            info!("Downloading Ollama installer from: {} (attempt {}/{})", download_url, attempt, MAX_RETRIES);
            
            match self.download_installer_with_verification(download_url, &installer_path).await {
                Ok(_) => break,
                Err(e) => {
                    warn!("Download attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < MAX_RETRIES {
                        info!("Retrying download in 2 seconds...");
                        sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        }
        
        // If all retries failed, return the last error
        if let Some(error) = last_error {
            return Err(error);
        }
        
        info!("Running Ollama installer");
        
        // Run the installer silently
        let output = Command::new(&installer_path)
            .args(&["/S"]) // Silent install
            .output()
            .map_err(|e| AppError::OllamaError(format!("Failed to run installer: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::OllamaError(
                format!("Ollama installation failed: {}", stderr)
            ));
        }
        
        // Clean up installer
        let _ = std::fs::remove_file(&installer_path);
        
        info!("Ollama installed successfully on Windows");
        
        // Give it a moment to finish installation
        sleep(Duration::from_secs(3)).await;
        
        Ok(())
    }
    
    async fn download_installer_with_verification(&self, url: &str, path: &std::path::Path) -> AppResult<()> {
        // Download the installer
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::OllamaError(format!("Failed to download installer: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(AppError::OllamaError(
                format!("Failed to download installer: HTTP {}", response.status())
            ));
        }
        
        // Get content length for verification
        let content_length = response.content_length();
        
        // Read installer bytes
        let installer_bytes = response.bytes().await
            .map_err(|e| AppError::OllamaError(format!("Failed to read installer: {}", e)))?;
        
        // Verify download integrity
        self.verify_installer_integrity(&installer_bytes, content_length)?;
        
        // Store length before moving bytes
        let bytes_len = installer_bytes.len();
        
        // Save installer to temp file
        std::fs::write(path, installer_bytes)
            .map_err(|e| AppError::OllamaError(format!("Failed to save installer: {}", e)))?;
        
        info!("Installer downloaded and verified successfully ({} bytes)", bytes_len);
        Ok(())
    }
    
    fn verify_installer_integrity(&self, bytes: &[u8], expected_length: Option<u64>) -> AppResult<()> {
        // Basic size check - installer should be at least 1MB
        const MIN_INSTALLER_SIZE: usize = 1024 * 1024; // 1MB
        const MAX_INSTALLER_SIZE: usize = 500 * 1024 * 1024; // 500MB
        
        if bytes.len() < MIN_INSTALLER_SIZE {
            return Err(AppError::OllamaError(
                format!("Downloaded installer appears corrupted (too small: {} bytes, expected at least {} bytes)", 
                    bytes.len(), MIN_INSTALLER_SIZE)
            ));
        }
        
        if bytes.len() > MAX_INSTALLER_SIZE {
            return Err(AppError::OllamaError(
                format!("Downloaded installer appears corrupted (too large: {} bytes, expected at most {} bytes)", 
                    bytes.len(), MAX_INSTALLER_SIZE)
            ));
        }
        
        // Verify content length matches if provided
        if let Some(expected) = expected_length {
            if bytes.len() != expected as usize {
                return Err(AppError::OllamaError(
                    format!("Downloaded installer size mismatch: got {} bytes, expected {} bytes", 
                        bytes.len(), expected)
                ));
            }
        }
        
        // Check for executable signature (Windows PE header)
        if bytes.len() >= 2 && &bytes[0..2] != b"MZ" {
            return Err(AppError::OllamaError(
                "Downloaded file does not appear to be a valid Windows executable".to_string()
            ));
        }
        
        info!("Installer integrity verification passed");
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
        
        if !response.status().is_success() {
            return Err(AppError::OllamaError(
                format!("Failed to start model download: HTTP {}", response.status())
            ));
        }
        
        // Process streaming response with robust error handling
        let mut parse_errors = 0;
        const MAX_PARSE_ERRORS: usize = 10;
        
        while let Some(chunk_result) = response.chunk().await.transpose() {
            match chunk_result {
                Ok(chunk_bytes) => {
                    match std::str::from_utf8(&chunk_bytes) {
                        Ok(text) => {
                            // Parse each line as JSON with error recovery
                            for line in text.lines() {
                                let line = line.trim();
                                if line.is_empty() {
                                    continue;
                                }
                                
                                match serde_json::from_str::<serde_json::Value>(line) {
                                    Ok(json) => {
                                        // Reset parse error counter on successful parse
                                        parse_errors = 0;
                                        
                                        if let Some(status) = json["status"].as_str() {
                                            let total = json["total"].as_u64().unwrap_or(100) as f32;
                                            let completed = json["completed"].as_u64().unwrap_or(0) as f32;
                                            let progress = if total > 0.0 { completed / total } else { 0.0 };
                                            progress_callback(progress.clamp(0.0, 1.0), status.to_string());
                                        }
                                        
                                        // Check for error in the JSON response
                                        if let Some(error) = json["error"].as_str() {
                                            return Err(AppError::OllamaError(
                                                format!("Ollama download error: {}", error)
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        parse_errors += 1;
                                        warn!("Failed to parse streaming response line: '{}' - Error: {}", line, e);
                                        
                                        // If we get too many parse errors, something is seriously wrong
                                        if parse_errors >= MAX_PARSE_ERRORS {
                                            return Err(AppError::OllamaError(
                                                format!("Too many JSON parse errors ({}), aborting download", parse_errors)
                                            ));
                                        }
                                        
                                        // Continue processing other lines
                                        continue;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to decode chunk as UTF-8: {}", e);
                            // Continue processing, this might be a partial UTF-8 sequence
                            continue;
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading response chunk: {}", e);
                    return Err(AppError::OllamaError(
                        format!("Network error during download: {}", e)
                    ));
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