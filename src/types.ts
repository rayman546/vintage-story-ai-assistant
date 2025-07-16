export interface ChatMessage {
  id: string;
  content: string;
  role: 'user' | 'assistant' | 'error';
  timestamp: string;
}

export interface ChatSession {
  id: string;
  title: string;
  timestamp: string;
  messages: ChatMessage[];
}

export interface ChatResponse {
  message: ChatMessage;
  context_used: string[];
}

export interface OllamaStatus {
  is_running: boolean;
  is_installed: boolean;
  version?: string;
  models: ModelInfo[];
}

export interface ModelInfo {
  name: string;
  size: number;
  digest: string;
  details: {
    parameter_size: string;
    quantization_level: string;
    family: string;
  };
}

export interface WikiStatus {
  last_update?: string;
  total_pages: number;
  is_updating: boolean;
  pages_scraped: number;
  errors_encountered: number;
}

export interface SystemStatus {
  ollama_ready: boolean;
  wiki_ready: boolean;
  error_message?: string;
}