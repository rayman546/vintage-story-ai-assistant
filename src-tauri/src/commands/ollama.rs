use crate::AppState;
use crate::services::ollama_manager::{OllamaStatus, ModelInfo};
use crate::commands::validation::validate_model_name;
use tauri::State;

#[tauri::command]
pub async fn check_ollama_status(state: State<'_, AppState>) -> Result<OllamaStatus, String> {
    let ollama_manager = state.ollama_manager.lock().await;
    ollama_manager.get_status().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_ollama(state: State<'_, AppState>) -> Result<String, String> {
    let mut ollama_manager = state.ollama_manager.lock().await;
    ollama_manager.ensure_available().await.map_err(|e| e.to_string())?;
    Ok("Ollama installed and ready".to_string())
}

#[tauri::command]
pub async fn start_ollama(state: State<'_, AppState>) -> Result<String, String> {
    let mut ollama_manager = state.ollama_manager.lock().await;
    ollama_manager.start_service().await.map_err(|e| e.to_string())?;
    Ok("Ollama service started successfully".to_string())
}

#[tauri::command]
pub async fn download_model(state: State<'_, AppState>, model_name: String) -> Result<String, String> {
    // Validate model name before attempting download
    validate_model_name(&model_name).map_err(|e| e.to_string())?;
    
    let ollama_manager = state.ollama_manager.lock().await;
    ollama_manager.download_model(&model_name).await.map_err(|e| e.to_string())?;
    Ok(format!("Model {} downloaded successfully", model_name))
}

#[tauri::command]
pub async fn list_models(state: State<'_, AppState>) -> Result<Vec<ModelInfo>, String> {
    let ollama_manager = state.ollama_manager.lock().await;
    ollama_manager.list_models().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ensure_ollama_ready(state: State<'_, AppState>) -> Result<OllamaStatus, String> {
    let mut ollama_manager = state.ollama_manager.lock().await;
    
    // Try to ensure Ollama is available
    if let Err(e) = ollama_manager.ensure_available().await {
        log::error!("Failed to ensure Ollama is ready: {}", e);
        // Return status anyway so frontend knows what's wrong
    }
    
    ollama_manager.get_status().await.map_err(|e| e.to_string())
}
