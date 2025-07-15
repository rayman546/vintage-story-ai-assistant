pub mod ollama_manager;
pub mod wiki_service;
pub mod embedding_service;
pub mod chat_service;
pub mod vector_database;

#[cfg(test)]
#[path = "rag_integration_test.rs"]
mod rag_integration_test;

pub use ollama_manager::OllamaManager;
pub use wiki_service::WikiService;
pub use embedding_service::EmbeddingService;
pub use chat_service::ChatService;
pub use vector_database::VectorDatabase;
