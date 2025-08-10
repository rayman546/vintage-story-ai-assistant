# Task Specification: Fix Missing Embedding Model

## Problem Statement
The embedding service tries to use "nomic-embed-text" model with Ollama, but there's no mechanism to ensure this model is installed. The fallback to mock embeddings means the RAG system won't work properly in development or for users who haven't manually installed the model.

## Requirements
1. The system must automatically check for and install the required embedding model
2. Users should be informed when the model is being downloaded
3. The system should gracefully handle cases where the model cannot be installed
4. The embedding service should work consistently across all environments

## Implementation Plan

### 1. Update Embedding Service
- Modify `src-tauri/src/services/embedding_service.rs` to check for the required model
- Implement model installation functionality in the embedding service
- Add progress reporting for model downloads

### 2. Model Management
- Create a method to check if "nomic-embed-text" is available in Ollama
- Implement automatic download if the model is missing
- Add error handling for download failures

### 3. Progress Reporting
- Add callback mechanisms to report download progress to the frontend
- Update frontend to display model download progress
- Show appropriate status messages during model installation

### 4. Fallback Handling
- Improve fallback mechanism to clearly indicate when mock embeddings are being used
- Add user notifications when the system is operating in fallback mode
- Provide guidance on how to install the proper model

## Success Criteria
- [ ] Embedding model is automatically installed when missing
- [ ] Users receive clear feedback during model installation
- [ ] System works with real embeddings by default
- [ ] Graceful degradation to mock embeddings when necessary