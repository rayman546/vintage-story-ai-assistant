use crate::AppState;
use crate::services::chat_service::{ChatMessage, ChatResponse};
use tauri::State;

#[tauri::command]
pub async fn send_message(state: State<'_, AppState>, message: String) -> Result<ChatResponse, String> {
    let mut chat_service = state.chat_service.lock().await;
    chat_service.process_message(&message).await.map_err(|e| e.to_string())
}
