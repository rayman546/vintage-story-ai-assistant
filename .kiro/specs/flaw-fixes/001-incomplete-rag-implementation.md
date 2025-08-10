# Task Specification: Fix Incomplete RAG Implementation

## Problem Statement
The Retrieval-Augmented Generation (RAG) system has components for wiki scraping, embedding generation, and vector storage, but they're not fully integrated. The chat service doesn't properly utilize the RAG pipeline to enhance responses with contextual information from the wiki.

## Requirements
1. The chat service must retrieve relevant context from the vector database before generating responses
2. Context should be properly formatted and included in the prompt sent to Ollama
3. The system should display which sources were used to generate the response
4. The RAG pipeline should work end-to-end without errors

## Implementation Plan

### 1. Update Chat Service to Use RAG
- Modify `src-tauri/src/services/chat_service.rs` to properly integrate with the embedding service
- Implement context retrieval before generating LLM responses
- Update the `process_message` method to:
  1. Retrieve similar documents from the vector database
  2. Format context appropriately for the prompt
  3. Include source information in the response

### 2. Enhance Prompt Construction
- Update the `build_prompt` method in `chat_service.rs` to include retrieved context
- Ensure context is clearly separated in the prompt
- Add instructions for the LLM to use the context when appropriate

### 3. Improve Response Structure
- Modify `ChatResponse` to include context sources information
- Update frontend to display sources used in responses

### 4. Testing
- Verify that context is being retrieved and used in responses
- Test with various queries to ensure relevant context is found
- Confirm that source information is properly displayed

## Success Criteria
- [ ] Chat responses include relevant information from the wiki
- [ ] Source information is displayed with each response
- [ ] Context retrieval works without errors
- [ ] System performance is acceptable (response time < 10 seconds)