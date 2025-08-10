# Task Specification: Remove Hardcoded Values

## Problem Statement
Several values like the embedding model name ("nomic-embed-text") and chunk sizes are hardcoded, making the system less flexible.

## Requirements
1. All configurable values should be defined in configuration files
2. Users should be able to modify these values without code changes
3. Default values should be sensible for most users
4. The system should validate configuration values

## Implementation Plan

### 1. Configuration Structure
- Add configuration options for embedding model name in `AppConfig`
- Make chunk size and overlap configurable in `EmbeddingConfig`
- Add other hardcoded values to appropriate configuration structs

### 2. Configuration Loading
- Update services to load values from configuration instead of using hardcoded values
- Implement default values for all configuration options
- Add validation for configuration values

### 3. User Interface
- Add UI controls for configurable values in the Settings panel
- Group related settings logically
- Provide descriptions for each setting

### 4. Documentation
- Document all configuration options
- Provide guidance on when to modify each setting
- Include recommended values for different use cases

## Success Criteria
- [ ] All configurable values are defined in configuration files
- [ ] Users can modify values without code changes
- [ ] Default values are sensible for most users
- [ ] System validates configuration values