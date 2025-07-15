#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EmbeddingConfig;
    use crate::services::embedding_service::EmbeddingService;
    use mockito::{Server, ServerGuard, Matcher};
    use serde_json::json;

    async fn create_test_service() -> (EmbeddingService, ServerGuard) {
        let mut server = Server::new();
        let mut service = EmbeddingService::new().await;
        
        // Override the config to use mockito server
        let url = server.url();
        let parts: Vec<&str> = url.trim_start_matches("http://").split(':').collect();
        
        (service, server)
    }

    #[tokio::test]
    async fn test_split_into_chunks() {
        let (service, _server) = create_test_service().await;
        
        // Test with content that should be split into multiple chunks
        let content = "This is a test sentence. ".repeat(100);
        let chunks = service.split_into_chunks(&content);
        
        assert!(!chunks.is_empty());
        assert!(chunks[0].len() <= service.config.chunk_size);
        
        // Verify overlap between chunks
        if chunks.len() > 1 {
            // Check that there's some overlap
            let first_chunk_words: Vec<&str> = chunks[0].split_whitespace().collect();
            let second_chunk_words: Vec<&str> = chunks[1].split_whitespace().collect();
            
            // The end of first chunk should overlap with beginning of second
            let overlap_exists = first_chunk_words.iter()
                .rev()
                .take(10)
                .any(|word| second_chunk_words.contains(word));
            
            assert!(overlap_exists, "Chunks should have overlap");
        }
    }

    #[tokio::test]
    async fn test_sanitize_title() {
        let (service, _server) = create_test_service().await;
        
        assert_eq!(service.sanitize_title("Hello World!"), "hello_world");
        assert_eq!(service.sanitize_title("Test@#$123"), "test123");
        assert_eq!(service.sanitize_title("Multiple   Spaces"), "multiple___spaces");
    }
}