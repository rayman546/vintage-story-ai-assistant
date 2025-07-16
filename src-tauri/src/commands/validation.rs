use crate::errors::{AppError, AppResult};

/// Validates a model name for format and character constraints
/// 
/// # Arguments
/// * `name` - The model name to validate
/// 
/// # Returns
/// * `AppResult<()>` - Ok if valid, Err with specific validation error if invalid
/// 
/// # Validation Rules
/// - Cannot be empty
/// - Maximum length of 100 characters
/// - Only alphanumeric characters, colons, hyphens, underscores, and dots allowed
/// - Must not start or end with special characters
pub fn validate_model_name(name: &str) -> AppResult<()> {
    // Check if empty
    if name.is_empty() {
        return Err(AppError::ConfigError("Model name cannot be empty".to_string()));
    }
    
    // Check length limit
    if name.len() > 100 {
        return Err(AppError::ConfigError(
            "Model name too long (maximum 100 characters)".to_string()
        ));
    }
    
    // Check for valid characters (alphanumeric, :, -, _, .)
    if !name.chars().all(|c| c.is_alphanumeric() || matches!(c, ':' | '-' | '_' | '.')) {
        return Err(AppError::ConfigError(
            "Model name contains invalid characters. Only letters, numbers, colons (:), hyphens (-), underscores (_), and dots (.) are allowed".to_string()
        ));
    }
    
    // Check that it doesn't start or end with special characters
    let first_char = name.chars().next().unwrap();
    let last_char = name.chars().last().unwrap();
    
    if !first_char.is_alphanumeric() {
        return Err(AppError::ConfigError(
            "Model name must start with a letter or number".to_string()
        ));
    }
    
    if !last_char.is_alphanumeric() {
        return Err(AppError::ConfigError(
            "Model name must end with a letter or number".to_string()
        ));
    }
    
    Ok(())
}

/// Validates message content for length and basic format constraints
/// 
/// # Arguments
/// * `content` - The message content to validate
/// 
/// # Returns
/// * `AppResult<()>` - Ok if valid, Err with specific validation error if invalid
/// 
/// # Validation Rules
/// - Cannot be empty or only whitespace
/// - Maximum length of 10,000 characters
/// - Must contain at least one non-whitespace character
pub fn validate_message_content(content: &str) -> AppResult<()> {
    // Check if empty or only whitespace
    let trimmed_content = content.trim();
    if trimmed_content.is_empty() {
        return Err(AppError::ConfigError(
            "Message cannot be empty or contain only whitespace".to_string()
        ));
    }
    
    // Check length limit
    if content.len() > 10000 {
        return Err(AppError::ConfigError(
            "Message too long (maximum 10,000 characters)".to_string()
        ));
    }
    
    // Additional validation: check for potentially problematic characters
    // Allow most Unicode characters but reject control characters except common ones
    let has_invalid_chars = content.chars().any(|c| {
        c.is_control() && !matches!(c, '\n' | '\r' | '\t')
    });
    
    if has_invalid_chars {
        return Err(AppError::ConfigError(
            "Message contains invalid control characters".to_string()
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_model_name_valid() {
        // Valid model names
        assert!(validate_model_name("llama2").is_ok());
        assert!(validate_model_name("llama2:7b").is_ok());
        assert!(validate_model_name("codellama:13b-instruct").is_ok());
        assert!(validate_model_name("mistral:7b-instruct-v0.1").is_ok());
        assert!(validate_model_name("phi:2.7b").is_ok());
    }

    #[test]
    fn test_validate_model_name_invalid() {
        // Empty name
        assert!(validate_model_name("").is_err());
        
        // Too long
        let long_name = "a".repeat(101);
        assert!(validate_model_name(&long_name).is_err());
        
        // Invalid characters
        assert!(validate_model_name("model@name").is_err());
        assert!(validate_model_name("model name").is_err());
        assert!(validate_model_name("model/name").is_err());
        
        // Starting/ending with special characters
        assert!(validate_model_name(":model").is_err());
        assert!(validate_model_name("model:").is_err());
        assert!(validate_model_name("-model").is_err());
        assert!(validate_model_name("model-").is_err());
    }

    #[test]
    fn test_validate_message_content_valid() {
        // Valid messages
        assert!(validate_message_content("Hello world").is_ok());
        assert!(validate_message_content("How do I craft a pickaxe?").is_ok());
        assert!(validate_message_content("Multi\nline\nmessage").is_ok());
        assert!(validate_message_content("Message with\ttabs").is_ok());
        
        // Unicode characters
        assert!(validate_message_content("Hello 世界").is_ok());
        assert!(validate_message_content("Café").is_ok());
    }

    #[test]
    fn test_validate_message_content_invalid() {
        // Empty or whitespace only
        assert!(validate_message_content("").is_err());
        assert!(validate_message_content("   ").is_err());
        assert!(validate_message_content("\n\t  \r").is_err());
        
        // Too long
        let long_message = "a".repeat(10001);
        assert!(validate_message_content(&long_message).is_err());
        
        // Control characters (except allowed ones)
        assert!(validate_message_content("Hello\x00world").is_err());
        assert!(validate_message_content("Hello\x1Bworld").is_err());
    }

    #[test]
    fn test_validate_message_content_edge_cases() {
        // Exactly at limit
        let max_length_message = "a".repeat(10000);
        assert!(validate_message_content(&max_length_message).is_ok());
        
        // Just over limit
        let over_limit_message = "a".repeat(10001);
        assert!(validate_message_content(&over_limit_message).is_err());
        
        // Whitespace at edges but valid content
        assert!(validate_message_content("  valid content  ").is_ok());
    }
}