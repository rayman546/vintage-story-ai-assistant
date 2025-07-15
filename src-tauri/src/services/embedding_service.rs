use crate::config::EmbeddingConfig;
use crate::errors::{AppError, AppResult};
use crate::services::vector_database::{VectorDatabase, VectorDocument};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChunk {
    pub id: String,
    pub content: String,
    pub source_url: String,
    pub source_title: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub chunk: TextChunk,
    pub similarity_score: f32,
}

pub struct EmbeddingService {
    config: EmbeddingConfig,
    chunks: Vec<TextChunk>,
    client: Client,
    vector_db: Arc<Mutex<VectorDatabase>>,
}

impl EmbeddingService {
    pub async fn new() -> Self {
        let config = EmbeddingConfig::default();
        let client = Client::new();
        
        // Initialize vector database
        let vector_db = match VectorDatabase::new().await {
            Ok(db) => {
                // Initialize the database tables
                if let Err(e) = db.initialize().await {
                    error!("Failed to initialize vector database: {}", e);
                }
                Arc::new(Mutex::new(db))
            }
            Err(e) => {
                error!("Failed to create vector database: {}", e);
                // For development, create a dummy database that will gracefully handle failures
                // This allows the app to start even if the database is locked
                warn!("Creating fallback vector database due to initialization failure");
                Arc::new(Mutex::new(VectorDatabase::new_fallback()))
            }
        };
        
        Self {
            config,
            chunks: Vec::new(),
            client,
            vector_db,
        }
    }
    
    pub async fn process_wiki_page(&mut self, title: &str, url: &str, content: &str) -> AppResult<()> {
        info!("Processing wiki page for embeddings: {}", title);
        
        // Split content into chunks
        let chunks = self.split_into_chunks(content);
        let total_chunks = chunks.len();
        
        // Process chunks in batches for efficiency
        let batch_size = self.config.batch_size;
        let mut processed = 0;
        
        for batch_start in (0..chunks.len()).step_by(batch_size) {
            let batch_end = std::cmp::min(batch_start + batch_size, chunks.len());
            let batch = &chunks[batch_start..batch_end];
            
            // Generate embeddings for batch
            let mut batch_chunks = Vec::new();
            for (i, chunk_content) in batch.iter().enumerate() {
                if chunk_content.trim().len() < 50 {
                    continue; // Skip very short chunks
                }
                
                let chunk_index = batch_start + i;
                let chunk_id = format!("{}_{}", self.sanitize_title(title), chunk_index);
                
                match self.create_embedding(chunk_content).await {
                    Ok(embedding) => {
                        let mut metadata = HashMap::new();
                        metadata.insert("source_type".to_string(), "wiki".to_string());
                        metadata.insert("chunk_index".to_string(), chunk_index.to_string());
                        
                        let chunk = TextChunk {
                            id: chunk_id,
                            content: chunk_content.clone(),
                            source_url: url.to_string(),
                            source_title: title.to_string(),
                            embedding: Some(embedding),
                            metadata,
                        };
                        
                        batch_chunks.push(chunk);
                        processed += 1;
                    }
                    Err(e) => {
                        warn!("Failed to create embedding for chunk {}: {}", chunk_index, e);
                    }
                }
            }
            
            // Add batch to chunks
            self.chunks.extend(batch_chunks);
            
            info!("Processed {}/{} chunks for page: {}", processed, total_chunks, title);
            
            // Small delay between batches to avoid overwhelming the API
            if batch_end < chunks.len() {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        
        // Save all processed chunks to the database
        if !self.chunks.is_empty() {
            info!("Saving {} chunks to vector database", self.chunks.len());
            
            // Convert TextChunks to VectorDocuments
            let documents: Vec<VectorDocument> = self.chunks
                .iter()
                .filter_map(|chunk| {
                    if let Some(ref embedding) = chunk.embedding {
                        Some(VectorDocument {
                            id: chunk.id.clone(),
                            content: chunk.content.clone(),
                            source_url: chunk.source_url.clone(),
                            source_title: chunk.source_title.clone(),
                            embedding: embedding.clone(),
                            metadata: serde_json::to_string(&chunk.metadata).unwrap_or_default(),
                        })
                    } else {
                        None
                    }
                })
                .collect();
            
            // Save to database
            let db = self.vector_db.lock().await;
            if let Err(e) = db.insert_documents(documents).await {
                error!("Failed to save chunks to database: {}", e);
            } else {
                info!("Successfully saved chunks to database");
            }
        }
        
        info!("Created {} embeddings from {} chunks for page: {}", processed, total_chunks, title);
        Ok(())
    }
    
    pub async fn embed_text(&self, text: &str) -> AppResult<Vec<f32>> {
        self.create_embedding(text).await
    }
    
    pub async fn search_similar(&self, query: &str, limit: usize) -> AppResult<Vec<SimilarityResult>> {
        let query_embedding = self.create_embedding(query).await?;
        
        // Search in vector database
        let db = self.vector_db.lock().await;
        let db_results = db.search_similar(query_embedding.clone(), limit).await?;
        
        // Convert database results to SimilarityResult
        let mut results = Vec::new();
        for (doc, score) in db_results {
            let chunk = TextChunk {
                id: doc.id,
                content: doc.content,
                source_url: doc.source_url,
                source_title: doc.source_title,
                embedding: None, // Don't need to return embeddings
                metadata: serde_json::from_str(&doc.metadata).unwrap_or_default(),
            };
            
            results.push(SimilarityResult {
                chunk,
                similarity_score: score,
            });
        }
        
        // If no results from database, fall back to in-memory search
        if results.is_empty() && !self.chunks.is_empty() {
            warn!("No results from database, falling back to in-memory search");
            let mut memory_results: Vec<SimilarityResult> = self.chunks
                .iter()
                .filter_map(|chunk| {
                    if let Some(ref embedding) = chunk.embedding {
                        let similarity = self.cosine_similarity(&query_embedding, embedding);
                        Some(SimilarityResult {
                            chunk: chunk.clone(),
                            similarity_score: similarity,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            
            // Sort by similarity score (highest first)
            memory_results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
            
            // Return top results
            memory_results.truncate(limit);
            return Ok(memory_results);
        }
        
        Ok(results)
    }
    
    async fn create_embedding(&self, text: &str) -> AppResult<Vec<f32>> {
        // Try to call Ollama's embedding API first
        let url = "http://localhost:11434/api/embeddings";
        
        let payload = serde_json::json!({
            "model": "nomic-embed-text",
            "prompt": text
        });
        
        match self.client
            .post(url)
            .json(&payload)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(result) => {
                            // Extract embedding from response
                            if let Some(embedding_array) = result["embedding"].as_array() {
                                let embedding: Vec<f32> = embedding_array
                                    .iter()
                                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                                    .collect();
                                
                                if !embedding.is_empty() {
                                    return Ok(embedding);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse Ollama embedding response: {}", e);
                        }
                    }
                } else {
                    warn!("Ollama embedding API returned status: {}", response.status());
                }
            }
            Err(e) => {
                warn!("Failed to call Ollama embedding API: {}", e);
            }
        }
        
        // Fall back to mock embeddings for development
        info!("Using mock embeddings for development (Ollama not available)");
        self.create_mock_embedding(text)
    }
    
    fn create_mock_embedding(&self, text: &str) -> AppResult<Vec<f32>> {
        // Create a simple but deterministic "embedding" based on text characteristics
        // This is just for development - replace with real embeddings later
        let mut embedding = vec![0.0; 384]; // Standard embedding size
        
        let words: Vec<&str> = text.split_whitespace().collect();
        let char_count = text.len() as f32;
        let word_count = words.len() as f32;
        
        // Fill embedding with simple features
        for (i, word) in words.iter().enumerate().take(100) {
            let hash = self.simple_hash(word) as usize;
            let index = hash % embedding.len();
            embedding[index] += 1.0 / word_count;
        }
        
        // Add some basic text statistics
        if embedding.len() > 10 {
            embedding[0] = char_count / 1000.0; // Normalized character count
            embedding[1] = word_count / 100.0;  // Normalized word count
            embedding[2] = text.matches('.').count() as f32 / 10.0; // Sentence density
        }
        
        // Normalize the embedding vector
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in &mut embedding {
                *value /= magnitude;
            }
        }
        
        Ok(embedding)
    }
    
    fn simple_hash(&self, text: &str) -> u32 {
        let mut hash = 0u32;
        for byte in text.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }
    
    fn cosine_similarity(&self, vec_a: &[f32], vec_b: &[f32]) -> f32 {
        if vec_a.len() != vec_b.len() {
            return 0.0;
        }
        
        let dot_product: f32 = vec_a.iter().zip(vec_b.iter()).map(|(a, b)| a * b).sum();
        let magnitude_a: f32 = vec_a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = vec_b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            0.0
        } else {
            dot_product / (magnitude_a * magnitude_b)
        }
    }
    
    pub fn split_into_chunks(&self, content: &str) -> Vec<String> {
        let chunk_size = self.config.chunk_size;
        let overlap = self.config.chunk_overlap;
        
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut chunks = Vec::new();
        
        if words.len() <= chunk_size {
            chunks.push(content.to_string());
            return chunks;
        }
        
        let mut start = 0;
        while start < words.len() {
            let end = std::cmp::min(start + chunk_size, words.len());
            let chunk = words[start..end].join(" ");
            
            if !chunk.trim().is_empty() {
                chunks.push(chunk);
            }
            
            if end >= words.len() {
                break;
            }
            
            start = end - overlap;
        }
        
        chunks
    }
    
    fn sanitize_title(&self, title: &str) -> String {
        title.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .replace(' ', "_")
            .to_lowercase()
    }
    
    pub fn get_chunk_count(&self) -> usize {
        self.chunks.len()
    }
    
    pub fn get_chunks_for_source(&self, source_url: &str) -> Vec<&TextChunk> {
        self.chunks.iter().filter(|chunk| chunk.source_url == source_url).collect()
    }
}

#[cfg(test)]
#[path = "embedding_service_test.rs"]
mod tests;
