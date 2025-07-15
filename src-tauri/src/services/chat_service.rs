use crate::errors::AppResult;
use crate::services::embedding_service::{EmbeddingService, SimilarityResult};
use crate::services::ollama_manager::OllamaManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub content: String,
    pub role: String, // "user" or "assistant"
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub context_used: Vec<String>,
}

pub struct ChatService {
    embedding_service: Arc<Mutex<EmbeddingService>>,
    ollama_manager: Arc<Mutex<OllamaManager>>,
    conversation_history: Vec<ChatMessage>,
}

impl ChatService {
    pub async fn new() -> Self {
        let embedding_service = Arc::new(Mutex::new(EmbeddingService::new().await));
        let ollama_manager = Arc::new(Mutex::new(OllamaManager::new().await));
        
        Self {
            embedding_service,
            ollama_manager,
            conversation_history: Vec::new(),
        }
    }
    
    pub fn set_embedding_service(&mut self, embedding_service: Arc<Mutex<EmbeddingService>>) {
        self.embedding_service = embedding_service;
    }
    
    pub fn set_ollama_manager(&mut self, ollama_manager: Arc<Mutex<OllamaManager>>) {
        self.ollama_manager = ollama_manager;
    }

    pub async fn process_message(&mut self, message: &str) -> AppResult<ChatResponse> {
        info!("Processing user message: {}", message);
        
        // Store user message in history
        let user_message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: message.to_string(),
            role: "user".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        self.conversation_history.push(user_message);
        
        // Search for relevant context using embedding service
        let context_results = {
            let embedding_service = self.embedding_service.lock().await;
            embedding_service.search_similar(message, 5).await.unwrap_or_default()
        };
        
        // Extract context text and sources
        let context_texts: Vec<String> = context_results.iter()
            .map(|result| format!("Source: {}\n{}", result.chunk.source_title, result.chunk.content))
            .collect();
        
        let context_sources: Vec<String> = context_results.iter()
            .map(|result| format!("{} (score: {:.2})", result.chunk.source_title, result.similarity_score))
            .collect();
        
        // Generate response using Ollama with context
        let response_content = self.generate_llm_response(message, &context_texts).await?;
        
        // Create assistant message
        let assistant_message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: response_content,
            role: "assistant".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        // Store assistant message in history
        self.conversation_history.push(assistant_message.clone());
        
        Ok(ChatResponse {
            message: assistant_message,
            context_used: context_sources,
        })
    }
    
    async fn generate_llm_response(&self, query: &str, context: &[String]) -> AppResult<String> {
        // Build prompt with context
        let prompt = self.build_prompt(query, context);
        
        // Call Ollama to generate response
        let ollama = self.ollama_manager.lock().await;
        
        match ollama.generate_response(&prompt).await {
            Ok(response) => Ok(response),
            Err(e) => {
                error!("Failed to generate LLM response: {}", e);
                // Fall back to a simple response if LLM fails
                Ok(self.generate_fallback_response(query))
            }
        }
    }
    
    fn build_prompt(&self, query: &str, context: &[String]) -> String {
        let mut prompt = String::from("You are a helpful assistant specializing in the game Vintage Story. You provide accurate, detailed information based on the game's wiki and mechanics.\n\n");
        
        // Add context if available
        if !context.is_empty() {
            prompt.push_str("Here is relevant information from the Vintage Story wiki:\n\n");
            for (i, ctx) in context.iter().enumerate() {
                prompt.push_str(&format!("Context {}:\n{}\n\n", i + 1, ctx));
            }
            prompt.push_str("Based on the above context, ");
        }
        
        // Add conversation history for context
        if self.conversation_history.len() > 1 {
            prompt.push_str("Previous conversation:\n");
            // Include last 2-3 exchanges for context
            let start = self.conversation_history.len().saturating_sub(6);
            for msg in &self.conversation_history[start..] {
                prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
            }
            prompt.push_str("\n");
        }
        
        // Add the current query
        prompt.push_str(&format!("User question: {}\n\n", query));
        prompt.push_str("Assistant: Please provide a helpful and accurate response. If you have relevant context from the wiki, use it to give specific information. If you don't have specific information, provide general guidance about Vintage Story.");
        
        prompt
    }
    
    fn generate_fallback_response(&self, query: &str) -> String {
        let fallback_responses = vec![
            "I'm experiencing some technical difficulties connecting to the AI service. Could you please try again in a moment?",
            "I apologize, but I'm having trouble processing your request right now. Please try again shortly.",
            "The AI service is temporarily unavailable. In the meantime, you might want to check the Vintage Story wiki directly.",
        ];
        
        let index = query.len() % fallback_responses.len();
        fallback_responses[index].to_string()
    }
    
    pub fn get_conversation_history(&self) -> &[ChatMessage] {
        &self.conversation_history
    }
    
    pub fn clear_history(&mut self) {
        self.conversation_history.clear();
    }
}
