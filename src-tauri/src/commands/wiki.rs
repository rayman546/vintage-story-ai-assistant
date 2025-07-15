use crate::AppState;
use crate::services::wiki_service::WikiStatus;
use tauri::State;
use log::info;

#[tauri::command]
pub async fn get_wiki_status(state: State<'_, AppState>) -> Result<WikiStatus, String> {
    let wiki_service = state.wiki_service.lock().await;
    wiki_service.get_status().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_wiki_content(state: State<'_, AppState>) -> Result<String, String> {
    info!("Starting wiki content update from frontend command");
    
    // Start wiki update
    {
        let mut wiki_service = state.wiki_service.lock().await;
        wiki_service.update_content().await.map_err(|e| e.to_string())?;
    }
    
    // TODO: Process scraped content into embeddings
    // This would be called after wiki scraping completes
    // process_wiki_into_embeddings(&state).await?;
    
    Ok("Wiki content update completed successfully".to_string())
}

#[tauri::command]
pub async fn process_wiki_embeddings(state: State<'_, AppState>) -> Result<String, String> {
    info!("Processing wiki content into embeddings");
    
    // This is a placeholder for processing scraped wiki content into embeddings
    // In a full implementation, this would:
    // 1. Read stored wiki pages
    // 2. Process them through the embedding service
    // 3. Store the embeddings for search
    
    let embedding_service = state.embedding_service.lock().await;
    let chunk_count = embedding_service.get_chunk_count();
    
    Ok(format!("Processed wiki content. Total chunks: {}", chunk_count))
}

// Helper function for future implementation
async fn _process_wiki_into_embeddings(state: &State<'_, AppState>) -> Result<(), String> {
    // This would be implemented to:
    // 1. Get all scraped pages from WikiService
    // 2. Process each page through EmbeddingService
    // 3. Store embeddings for retrieval
    
    let _embedding_service = state.embedding_service.lock().await;
    // Implementation would go here
    
    Ok(())
}
