// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;

mod commands;
mod services;
mod config;
mod errors;

use services::{
    ollama_manager::OllamaManager,
    wiki_service::WikiService,
    embedding_service::EmbeddingService,
    chat_service::ChatService,
    vector_database::VectorDatabase,
};

/// Application state shared across all Tauri commands
#[derive(Clone)]
pub struct AppState {
    pub ollama_manager: Arc<Mutex<OllamaManager>>,
    pub wiki_service: Arc<Mutex<WikiService>>,
    pub embedding_service: Arc<Mutex<EmbeddingService>>,
    pub chat_service: Arc<Mutex<ChatService>>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    info!("Starting Vintage Story AI Assistant");

    // Initialize services
    let ollama_manager = Arc::new(Mutex::new(OllamaManager::new().await));
    let mut wiki_service = WikiService::new().await;
    let embedding_service = Arc::new(Mutex::new(EmbeddingService::new().await));
    
    // Connect wiki service to embedding service
    wiki_service.set_embedding_service(embedding_service.clone());
    let wiki_service = Arc::new(Mutex::new(wiki_service));
    
    // Create chat service and give it access to both services
    let mut chat_service = ChatService::new().await;
    chat_service.set_embedding_service(embedding_service.clone());
    chat_service.set_ollama_manager(ollama_manager.clone());
    let chat_service = Arc::new(Mutex::new(chat_service));

    let app_state = AppState {
        ollama_manager,
        wiki_service,
        embedding_service,
        chat_service,
    };

    // Build and run the Tauri application
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            commands::system::get_system_status,
            commands::ollama::check_ollama_status,
            commands::ollama::ensure_ollama_ready,
            commands::ollama::install_ollama,
            commands::ollama::start_ollama,
            commands::ollama::download_model,
            commands::ollama::list_models,
            commands::chat::send_message,
            commands::wiki::update_wiki_content,
            commands::wiki::get_wiki_status,
            commands::wiki::process_wiki_embeddings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
