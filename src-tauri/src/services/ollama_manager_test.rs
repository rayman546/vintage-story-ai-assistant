#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::OllamaConfig;
    use crate::errors::{AppError, AppResult};
    use crate::services::ollama_manager::OllamaManager;
    use mockito::{Server, ServerGuard, Matcher};
    use serde_json::json;

    async fn create_test_manager() -> (OllamaManager, ServerGuard) {
        let mut server = Server::new();
        let mut manager = OllamaManager::new().await;
        
        // Override the config to use mockito server
        let url = server.url();
        let parts: Vec<&str> = url.trim_start_matches("http://").split(':').collect();
        manager.config.host = parts[0].to_string();
        manager.config.port = parts[1].parse().unwrap();
        
        (manager, server)
    }

    #[tokio::test]
    async fn test_check_health_success() {
        let (manager, mut server) = create_test_manager().await;
        
        let _mock = server.mock("GET", "/api/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"models":[]}"#)
            .create();

        let result = manager.check_health().await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_health_failure() {
        let (manager, mut server) = create_test_manager().await;
        
        let _mock = server.mock("GET", "/api/tags")
            .with_status(500)
            .create();

        let result = manager.check_health().await;
        
        assert!(result.is_err());
        match result {
            Err(AppError::OllamaError(msg)) => {
                assert!(msg.contains("failed with status: 500"));
            }
            _ => panic!("Expected OllamaError"),
        }
    }
    #[tokio::test]
    async fn test_get_status() {
        let (manager, mut server) = create_test_manager().await;
        
        let models_response = json!({
            "models": [
                {
                    "name": "llama3.2:3b",
                    "size": 2000000000,
                    "digest": "abc123",
                    "details": {
                        "parameter_size": "3B",
                        "quantization_level": "Q4_0",
                        "family": "llama"
                    }
                }
            ]
        });

        let _health_mock = server.mock("GET", "/api/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(models_response.to_string())
            .expect(2) // Called twice - once for health check, once for listing models
            .create();

        let _version_mock = server.mock("GET", "/api/version")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"version":"0.1.0"}"#)
            .create();
        let status = manager.get_status().await.unwrap();
        
        assert!(status.is_running);
        assert_eq!(status.version, Some("0.1.0".to_string()));
        assert_eq!(status.models.len(), 1);
        assert_eq!(status.models[0].name, "llama3.2:3b");
    }

    #[tokio::test]
    async fn test_list_models() {
        let (manager, mut server) = create_test_manager().await;
        
        let models_response = json!({
            "models": [
                {
                    "name": "phi3:mini",
                    "size": 1500000000,
                    "digest": "def456",
                    "details": {
                        "parameter_size": "3.8B",
                        "quantization_level": "Q4_K_M",
                        "family": "phi"
                    }
                },
                {
                    "name": "nomic-embed-text",
                    "size": 500000000,
                    "digest": "ghi789",
                    "details": {
                        "parameter_size": "137M",
                        "quantization_level": "F16",
                        "family": "nomic"
                    }
                }
            ]
        });
        let _mock = server.mock("GET", "/api/tags")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(models_response.to_string())
            .create();

        let models = manager.list_models().await.unwrap();
        
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].name, "phi3:mini");
        assert_eq!(models[1].name, "nomic-embed-text");
        assert_eq!(models[0].details.parameter_size, "3.8B");
    }

    #[tokio::test]
    async fn test_generate_response() {
        let (mut manager, mut server) = create_test_manager().await;
        
        let response_json = json!({
            "model": "llama3.2:3b",
            "created_at": "2023-01-01T00:00:00Z",
            "response": "Hello! I'm an AI assistant for Vintage Story.",
            "done": true,
            "context": [1, 2, 3],
            "total_duration": 1000000000,
            "load_duration": 100000000,
            "prompt_eval_duration": 200000000,
            "eval_duration": 700000000,
            "eval_count": 10
        });
        let _mock = server.mock("POST", "/api/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response_json.to_string())
            .match_body(Matcher::AllOf(vec![
                Matcher::JsonString(r#"{"model":"llama3.2:3b"}"#.to_string()),
                Matcher::JsonString(r#"{"stream":false}"#.to_string()),
            ]))
            .create();

        manager.config.model_name = "llama3.2:3b".to_string();
        
        let response = manager.generate_response("Hello").await.unwrap();
        assert_eq!(response, "Hello! I'm an AI assistant for Vintage Story.");
    }

    #[tokio::test]
    async fn test_download_model() {
        let (manager, mut server) = create_test_manager().await;
        
        let _mock = server.mock("POST", "/api/pull")
            .with_status(200)
            .match_body(Matcher::Json(json!({
                "name": "phi3:mini"
            })))
            .create();

        let result = manager.download_model("phi3:mini").await;
        
        assert!(result.is_ok());
    }
}