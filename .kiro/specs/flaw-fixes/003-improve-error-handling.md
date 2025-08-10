# Task Specification: Improve Error Handling

## Problem Statement
Several critical operations lack proper error handling. For example, the wiki scraping service doesn't handle network failures gracefully, and the embedding service uses mock embeddings as a fallback without clear user notification.

## Requirements
1. All network operations must have proper error handling and retry mechanisms
2. Users must be informed when fallback mechanisms are being used
3. Error messages should be user-friendly and actionable
4. The system should attempt to recover from transient errors

## Implementation Plan

### 1. Network Error Handling
- Update `src-tauri/src/services/wiki_service.rs` to handle network failures gracefully
- Implement retry mechanisms for transient network errors
- Add timeouts for all network requests

### 2. User Notifications
- Modify frontend to display error messages from backend services
- Implement a notification system for user-facing errors
- Add status indicators for ongoing operations

### 3. Fallback Notifications
- Update `embedding_service.rs` to notify users when mock embeddings are being used
- Add visual indicators in the UI when fallback systems are active
- Provide clear information about reduced functionality in fallback mode

### 4. Error Recovery
- Implement retry logic for failed wiki scraping operations
- Add circuit breaker patterns for repeated failures
- Log detailed error information for debugging

## Success Criteria
- [ ] Network failures are handled gracefully with retry mechanisms
- [ ] Users are informed when fallback systems are active
- [ ] Error messages are clear and actionable
- [ ] System attempts to recover from transient errors