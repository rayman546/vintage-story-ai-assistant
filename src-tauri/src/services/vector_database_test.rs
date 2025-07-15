#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::AppResult;

    #[tokio::test]
    async fn test_vector_database_creation() -> AppResult<()> {
        let db = VectorDatabase::new().await?;
        db.initialize().await?;
        
        let count = db.count_documents().await?;
        assert_eq!(count, 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_and_search() -> AppResult<()> {
        let db = VectorDatabase::new().await?;
        db.initialize().await?;
        
        // Create test documents
        let docs = vec![
            VectorDocument {
                id: "doc1".to_string(),
                content: "How to craft a pickaxe in Vintage Story".to_string(),
                source_url: "test://wiki/crafting".to_string(),
                source_title: "Crafting Guide".to_string(),
                embedding: vec![1.0, 0.0, 0.0],
                metadata: "{}".to_string(),
            },
            VectorDocument {
                id: "doc2".to_string(),
                content: "Mining copper ore requires a pickaxe".to_string(),
                source_url: "test://wiki/mining".to_string(),
                source_title: "Mining Guide".to_string(),
                embedding: vec![0.8, 0.6, 0.0],
                metadata: "{}".to_string(),
            },
            VectorDocument {
                id: "doc3".to_string(),
                content: "Food preservation in Vintage Story".to_string(),
                source_url: "test://wiki/food".to_string(),
                source_title: "Food Guide".to_string(),
                embedding: vec![0.0, 0.0, 1.0],
                metadata: "{}".to_string(),
            },
        ];
        
        // Insert documents
        db.insert_documents(docs).await?;
        
        // Search with query embedding similar to first doc
        let query_embedding = vec![0.9, 0.1, 0.0];
        let results = db.search_similar(query_embedding, 2).await?;
        
        // Verify results
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0.id, "doc1");
        assert_eq!(results[1].0.id, "doc2");
        assert!(results[0].1 > results[1].1); // First should have higher similarity
        
        Ok(())
    }

    #[tokio::test]
    async fn test_cosine_similarity() {
        let db = VectorDatabase::new().await.unwrap();
        
        // Test identical vectors
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![1.0, 0.0, 0.0];
        assert_eq!(db.cosine_similarity(&vec1, &vec2), 1.0);
        
        // Test orthogonal vectors
        let vec3 = vec![0.0, 1.0, 0.0];
        assert_eq!(db.cosine_similarity(&vec1, &vec3), 0.0);
        
        // Test normalized vectors at 45 degrees
        let vec4 = vec![0.707, 0.707, 0.0];
        let similarity = db.cosine_similarity(&vec1, &vec4);
        assert!((similarity - 0.707).abs() < 0.001);
    }
}
