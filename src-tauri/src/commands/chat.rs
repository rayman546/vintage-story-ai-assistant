use crate::AppState;
use crate::services::chat_service::{ChatMessage, ChatResponse};
use crate::commands::validation::{validate_message_content, validate_model_name};
use tauri::State;

#[tauri::command]
pub async fn send_message(
    state: State<'_, AppState>, 
    message: String,
    model: Option<String>
) -> Result<ChatResponse, String> {
    // Validate message content
    validate_message_content(&message).map_err(|e| e.to_string())?;
    
    // Update the model if provided
    if let Some(model_name) = model {
        // Validate model name
        validate_model_name(&model_name).map_err(|e| e.to_string())?;
        
        let mut ollama_manager = state.ollama_manager.lock().await;
        ollama_manager.set_model(model_name);
    }
    
    let mut chat_service = state.chat_service.lock().await;
    chat_service.process_message(&message).await.map_err(|e| e.to_string())
}
