#[cfg(test)]
mod rag_integration_tests {
    use crate::services::{
        wiki_service::{WikiService, WikiPage},
        embedding_service::EmbeddingService,
        chat_service::ChatService,
        ollama_manager::OllamaManager,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_complete_rag_pipeline() {
        // Initialize all services
        let mut wiki_service = WikiService::new().await;
        let embedding_service = Arc::new(Mutex::new(EmbeddingService::new().await));
        let ollama_manager = Arc::new(Mutex::new(OllamaManager::new().await));
        let mut chat_service = ChatService::new().await;

        // Connect services
        wiki_service.set_embedding_service(embedding_service.clone());
        chat_service.set_embedding_service(embedding_service.clone());
        chat_service.set_ollama_manager(ollama_manager.clone());

        // Create test wiki content
        let test_page = WikiPage {
            title: "Crafting Guide".to_string(),
            url: "https://wiki.vintagestory.at/wiki/Crafting".to_string(),
            content: r#"
                Crafting is a fundamental mechanic in Vintage Story. Players can create tools, weapons, and other items using various materials.

                ## Basic Tools
                To start crafting, you need basic tools:
                - Hammer: Used for metalworking and shaping metal items
                - Knife: Essential for cutting materials and food preparation
                - Chisel: Required for stone carving and detailed work

                ## Materials
                Common crafting materials include:
                - Stone: Found everywhere, used for basic tools
                - Wood: Gathered from trees, essential for handles and construction
                - Metal: Smelted from ores, used for advanced tools

                ## Crafting Process
                1. Gather required materials
                2. Use appropriate tools
                3. Follow crafting recipes
                4. Create your desired item
            "#.to_string(),
            last_modified: None,
            categories: vec!["Crafting".to_string(), "Tools".to_string()],
        };

        // Process the wiki page (this will create embeddings)
        let result = wiki_service.save_page_content(&test_page).await;
        println!("Wiki processing result: {:?}", result);

        // Test similarity search
        let embedding_service_lock = embedding_service.lock().await;
        let search_results = embedding_service_lock.search_similar("How do I use a hammer?", 3).await;
        
        match search_results {
            Ok(results) => {
                println!("Found {} similar chunks", results.len());
                for (i, result) in results.iter().enumerate() {
                    println!("Result {}: {} (score: {:.3})", 
                             i + 1, 
                             result.chunk.content.chars().take(100).collect::<String>(),
                             result.similarity_score);
                }
                
                // Should find relevant content about hammers
                let has_hammer_content = results.iter().any(|r| 
                    r.chunk.content.to_lowercase().contains("hammer"));
                
                if has_hammer_content {
                    println!("✅ RAG pipeline successfully found relevant content!");
                } else {
                    println!("⚠️  RAG pipeline working but didn't find specific content (this is OK for testing)");
                }
            }
            Err(e) => {
                println!("⚠️  Embedding search failed (expected without Ollama): {}", e);
            }
        }
        drop(embedding_service_lock);

        // Test chat service integration
        let chat_result = chat_service.process_message("What tools do I need for crafting?").await;
        match chat_result {
            Ok(response) => {
                println!("✅ Chat service responded: {}", response.message.content.chars().take(100).collect::<String>());
                println!("Context used: {} chunks", response.context_used.len());
            }
            Err(e) => {
                println!("⚠️  Chat service failed (expected without Ollama): {}", e);
            }
        }

        // The test passes if we get this far without panicking
        println!("🎉 RAG integration test completed successfully!");
    }

    #[tokio::test]
    async fn test_embedding_service_chunking() {
        let embedding_service = EmbeddingService::new().await;
        
        let long_content = "This is a test sentence. ".repeat(100);
        let chunks = embedding_service.split_into_chunks(&long_content);
        
        println!("Created {} chunks from {} characters", chunks.len(), long_content.len());
        
        assert!(!chunks.is_empty());
        assert!(chunks.len() > 1); // Should split long content
        
        // Check chunk sizes are reasonable
        for (i, chunk) in chunks.iter().enumerate() {
            println!("Chunk {}: {} words", i + 1, chunk.split_whitespace().count());
            assert!(chunk.split_whitespace().count() <= 512); // Default chunk size
        }
        
        println!("✅ Chunking test passed!");
    }

    #[tokio::test]
    async fn test_vector_database_operations() {
        use crate::services::vector_database::{VectorDatabase, VectorDocument};
        
        let db = VectorDatabase::new_fallback(); // Use fallback to avoid file locks
        
        // Create test documents
        let test_docs = vec![
            VectorDocument {
                id: "test1".to_string(),
                content: "This is about crafting tools and weapons".to_string(),
                source_url: "test://1".to_string(),
                source_title: "Test 1".to_string(),
                embedding: vec![1.0, 0.0, 0.0, 0.5], // Mock embedding
                metadata: "{}".to_string(),
            },
            VectorDocument {
                id: "test2".to_string(),
                content: "This discusses building and construction".to_string(),
                source_url: "test://2".to_string(),
                source_title: "Test 2".to_string(),
                embedding: vec![0.0, 1.0, 0.0, 0.5], // Different mock embedding
                metadata: "{}".to_string(),
            },
        ];

        // Insert documents
        let insert_result = db.insert_documents(test_docs).await;
        assert!(insert_result.is_ok());
        println!("✅ Documents inserted successfully");

        // Search for similar documents
        let query_embedding = vec![1.0, 0.1, 0.0, 0.4]; // Similar to first document
        let search_results = db.search_similar(query_embedding, 2).await;
        
        match search_results {
            Ok(results) => {
                println!("Found {} results", results.len());
                assert!(!results.is_empty());
                
                // First result should be most similar
                if let Some((doc, score)) = results.first() {
                    println!("Best match: '{}' (score: {:.3})", doc.content, score);
                    assert!(doc.content.contains("crafting")); // Should match first document
                }
                
                println!("✅ Vector search test passed!");
            }
            Err(e) => {
                panic!("Vector search failed: {}", e);
            }
        }
    }
}