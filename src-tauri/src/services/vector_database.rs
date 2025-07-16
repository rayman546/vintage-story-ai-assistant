use crate::errors::{AppError, AppResult};
use crate::config::AppConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use log::{info, warn, error};
use sled::Db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: String,
    pub content: String,
    pub source_url: String,
    pub source_title: String,
    pub embedding: Vec<f32>,
    pub metadata: String,
}

pub struct VectorDatabase {
    db: Arc<Db>,
}

impl VectorDatabase {
    pub async fn new() -> AppResult<Self> {
        let data_dir = AppConfig::get_data_dir();
        let db_path = data_dir.join("vector_db");
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(&db_path)
            .map_err(|e| AppError::StorageError(format!("Failed to create vector DB directory: {}", e)))?;
        
        info!("Opening sled database at: {:?}", db_path);
        
        // Try to open the database with retry logic for lock issues
        let db = match sled::open(&db_path) {
            Ok(db) => db,
            Err(e) => {
                error!("Failed to open sled database: {}", e);
                
                // If it's a lock error, try to clean up and retry
                if e.to_string().contains("lock") || e.to_string().contains("locked") {
                    warn!("Database appears to be locked, attempting cleanup...");
                    
                    // Try to remove the entire database directory to clear locks
                    if db_path.exists() {
                        if let Err(cleanup_err) = std::fs::remove_dir_all(&db_path) {
                            warn!("Failed to cleanup database directory: {}", cleanup_err);
                        } else {
                            info!("Cleaned up locked database directory");
                        }
                        
                        // Recreate the directory
                        if let Err(create_err) = std::fs::create_dir_all(&db_path) {
                            warn!("Failed to recreate database directory: {}", create_err);
                        }
                    }
                    
                    // Try opening again with a fresh database
                    sled::open(&db_path)
                        .map_err(|e2| AppError::StorageError(format!("Failed to open sled database after cleanup: {}", e2)))?
                } else {
                    return Err(AppError::StorageError(format!("Failed to open sled database: {}", e)));
                }
            }
        };
        
        Ok(Self {
            db: Arc::new(db),
        })
    }
    
    pub fn new_fallback() -> Self {
        // Create an in-memory database as fallback
        let db = sled::Config::new().temporary(true).open()
            .expect("Failed to create temporary database");
        
        Self {
            db: Arc::new(db),
        }
    }
    
    pub async fn initialize(&self) -> AppResult<()> {
        info!("Vector database initialized");
        Ok(())
    }
    
    pub async fn insert_documents(&self, documents: Vec<VectorDocument>) -> AppResult<()> {
        if documents.is_empty() {
            return Ok(());
        }
        
        let mut batch = sled::Batch::default();
        
        for doc in &documents {
            let key = doc.id.as_bytes();
            let value = bincode::serialize(&doc)
                .map_err(|e| AppError::StorageError(format!("Failed to serialize document: {}", e)))?;
            
            batch.insert(key, value);
        }
        
        self.db.apply_batch(batch)
            .map_err(|e| AppError::StorageError(format!("Failed to insert batch: {}", e)))?;
        
        self.db.flush()
            .map_err(|e| AppError::StorageError(format!("Failed to flush database: {}", e)))?;
        
        info!("Inserted {} documents into vector database", documents.len());
        Ok(())
    }
    
    pub async fn search_similar(&self, embedding: Vec<f32>, limit: usize) -> AppResult<Vec<(VectorDocument, f32)>> {
        let mut results = Vec::new();
        
        // Iterate through all documents and calculate similarity
        for result in self.db.iter() {
            match result {
                Ok((_, value)) => {
                    if let Ok(doc) = bincode::deserialize::<VectorDocument>(&value) {
                        let similarity = self.cosine_similarity(&embedding, &doc.embedding);
                        results.push((doc, similarity));
                    }
                }
                Err(e) => {
                    error!("Error reading from database: {}", e);
                }
            }
        }
        
        // Sort by similarity (highest first)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Return top results
        results.truncate(limit);
        Ok(results)
    }
    
    pub async fn delete_by_source(&self, source_url: &str) -> AppResult<()> {
        let mut keys_to_delete = Vec::new();
        
        // Find all documents with matching source_url
        for result in self.db.iter() {
            match result {
                Ok((key, value)) => {
                    if let Ok(doc) = bincode::deserialize::<VectorDocument>(&value) {
                        if doc.source_url == source_url {
                            keys_to_delete.push(key);
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading from database: {}", e);
                }
            }
        }
        
        // Delete the documents
        let mut batch = sled::Batch::default();
        for key in keys_to_delete {
            batch.remove(key);
        }
        
        self.db.apply_batch(batch)
            .map_err(|e| AppError::StorageError(format!("Failed to delete documents: {}", e)))?;
        
        self.db.flush()
            .map_err(|e| AppError::StorageError(format!("Failed to flush database: {}", e)))?;
        
        info!("Deleted documents from source: {}", source_url);
        Ok(())
    }
    
    pub async fn count_documents(&self) -> AppResult<usize> {
        Ok(self.db.len())
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
}
