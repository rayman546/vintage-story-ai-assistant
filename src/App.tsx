import { useState, useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { 
  MessageCircle, Settings, Cpu, Download, Database, RefreshCw, 
  AlertCircle, CheckCircle, Loader, ChevronDown, Send, History
} from "lucide-react";
import { MessageRenderer } from "./components/MessageRenderer";
import { Settings as SettingsPanel } from "./components/Settings";
import { ChatHistory } from "./components/ChatHistory";
import { ErrorTester } from "./components/ErrorTester";
import { 
  ChatMessage, ChatSession, ChatResponse, OllamaStatus, 
  WikiStatus, SystemStatus 
} from "./types";



function App() {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [ollamaStatus, setOllamaStatus] = useState<OllamaStatus | null>(null);
  const [wikiStatus, setWikiStatus] = useState<WikiStatus | null>(null);
  const [systemStatus, setSystemStatus] = useState<SystemStatus>({
    ollama_ready: false,
    wiki_ready: false,
  });
  const [isInitializing, setIsInitializing] = useState(true);
  const [showSettings, setShowSettings] = useState(false);
  const [selectedModel, setSelectedModel] = useState("phi3:mini");
  const [showHistory, setShowHistory] = useState(false);
  const [temperature, setTemperature] = useState(0.7);
  const [maxContextChunks, setMaxContextChunks] = useState(5);
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [chatSessions, setChatSessions] = useState<ChatSession[]>([]);
  const [currentSessionId, setCurrentSessionId] = useState<string | null>(null);
  const messagesEndRef = useRef<null | HTMLDivElement>(null);

  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, []);

  const initializeSystem = useCallback(async () => {
    setIsInitializing(true);
    try {
      // Ensure Ollama is ready
      const ollamaReady = await invoke<OllamaStatus>("ensure_ollama_ready");
      setOllamaStatus(ollamaReady);
      
      // Check wiki status
      const wikiStat = await invoke<WikiStatus>("get_wiki_status");
      setWikiStatus(wikiStat);
      
      setSystemStatus({
        ollama_ready: ollamaReady.is_running && ollamaReady.models.length > 0,
        wiki_ready: wikiStat.total_pages > 0,
      });
      
      // Set default model if available
      if (ollamaReady.models && ollamaReady.models.length > 0) {
        setSelectedModel(ollamaReady.models[0].name);
      }
    } catch (error) {
      console.error("Failed to initialize system:", error);
      setSystemStatus({
        ollama_ready: false,
        wiki_ready: false,
        error_message: String(error),
      });
    } finally {
      setIsInitializing(false);
    }
  }, []);

  const updateWikiContent = useCallback(async () => {
    try {
      await invoke<string>("update_wiki_content");
      // Refresh status after update
      const status = await invoke<WikiStatus>("get_wiki_status");
      setWikiStatus(status);
      setSystemStatus(prev => ({ ...prev, wiki_ready: status.total_pages > 0 }));
    } catch (error) {
      console.error("Failed to update wiki content:", error);
    }
  }, []);

  useEffect(() => {
    initializeSystem();
  }, [initializeSystem]);

  useEffect(() => {
    scrollToBottom();
  }, [messages, scrollToBottom]);

  useEffect(() => {
    // Auto-save current session when messages change
    if (currentSessionId && messages.length > 0) {
      setChatSessions(prev => prev.map(session => 
        session.id === currentSessionId 
          ? { ...session, messages } 
          : session
      ));
    }
  }, [messages, currentSessionId]);

  const sendMessage = useCallback(async () => {
    if (!inputValue.trim() || !systemStatus.ollama_ready) return;

    const userMessage: ChatMessage = {
      id: Date.now().toString(),
      content: inputValue,
      role: "user",
      timestamp: new Date().toISOString(),
    };

    setMessages(prev => [...prev, userMessage]);
    setInputValue("");
    setIsLoading(true);

    try {
      const response = await invoke<ChatResponse>("send_message", {
        message: inputValue,
        model: selectedModel,
      });
      setMessages(prev => [...prev, response.message]);
    } catch (error) {
      console.error("Failed to send message:", error);
      const errorMessage: ChatMessage = {
        id: Date.now().toString(),
        content: `Error: ${error}`,
        role: "error",
        timestamp: new Date().toISOString(),
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  }, [inputValue, systemStatus.ollama_ready, selectedModel]);

  if (isInitializing) {
    return (
      <div className="flex h-screen items-center justify-center bg-gray-100">
        <div className="text-center">
          <Loader className="w-12 h-12 animate-spin mx-auto mb-4 text-blue-500" />
          <h2 className="text-xl font-semibold mb-2">Initializing Vintage Story AI Assistant</h2>
          <p className="text-gray-600">Setting up Ollama and loading knowledge base...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Sidebar */}
      <div className="w-64 bg-white shadow-lg">
        <div className="p-4 border-b">
          <h1 className="text-xl font-bold text-gray-800">
            Vintage Story AI
          </h1>
        </div>
        
        <div className="p-4">
          {/* System Status */}
          <div className="mb-6">
            <h2 className="text-sm font-semibold text-gray-600 mb-3">System Status</h2>
            <div className="space-y-3">
              <StatusItem
                icon={<Cpu className="w-4 h-4" />}
                label="Ollama"
                status={ollamaStatus?.is_running ? "Running" : "Stopped"}
                isReady={systemStatus.ollama_ready}
              />
              
              <StatusItem
                icon={<Database className="w-4 h-4" />}
                label="Knowledge Base"
                status={`${wikiStatus?.total_pages || 0} pages`}
                isReady={systemStatus.wiki_ready}
              />
            </div>
          </div>

          {/* Actions */}
          <div className="mb-6">
            <h2 className="text-sm font-semibold text-gray-600 mb-3">Actions</h2>
            <div className="space-y-2">
              {!systemStatus.ollama_ready && (
                <button
                  onClick={initializeSystem}
                  className="w-full text-left text-sm px-3 py-2 bg-red-50 hover:bg-red-100 rounded-lg flex items-center space-x-2"
                >
                  <AlertCircle className="w-4 h-4 text-red-600" />
                  <span>Setup Ollama</span>
                </button>
              )}
              
              <button
                onClick={updateWikiContent}
                disabled={wikiStatus?.is_updating}
                className="w-full text-left text-sm px-3 py-2 bg-blue-50 hover:bg-blue-100 rounded-lg disabled:opacity-50 flex items-center space-x-2"
              >
                {wikiStatus?.is_updating ? (
                  <RefreshCw className="w-4 h-4 animate-spin" />
                ) : (
                  <Download className="w-4 h-4" />
                )}
                <span>{wikiStatus?.is_updating ? "Updating..." : "Update Wiki"}</span>
              </button>
            </div>
          </div>

          {/* Model Selection */}
          {ollamaStatus?.models && ollamaStatus.models.length > 0 && (
            <div className="mb-6">
              <h2 className="text-sm font-semibold text-gray-600 mb-3">Model</h2>
              <div className="relative">
                <select
                  value={selectedModel}
                  onChange={(e) => setSelectedModel(e.target.value)}
                  className="w-full text-sm px-3 py-2 pr-8 bg-gray-50 rounded-lg appearance-none"
                >
                  {ollamaStatus.models.map((model) => (
                    <option key={model.name} value={model.name}>
                      {model.name}
                    </option>
                  ))}
                </select>
                <ChevronDown className="absolute right-2 top-1/2 transform -translate-y-1/2 w-4 h-4 pointer-events-none" />
              </div>
            </div>
          )}

          {/* Error Boundary Test (Development Only) */}
          {process.env.NODE_ENV === 'development' && (
            <div className="mb-6">
              <ErrorTester onTriggerError={() => {}} />
            </div>
          )}
        </div>
      </div>
      {/* Main Chat Area */}
      <div className="flex-1 flex flex-col">
        <div className="bg-white border-b p-4">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-semibold">Chat</h2>
            <div className="flex space-x-2">
              <button 
                onClick={() => setShowHistory(!showHistory)}
                className="p-2 hover:bg-gray-100 rounded-lg"
                title="Chat History"
              >
                <History className="w-5 h-5" />
              </button>
              <button 
                onClick={() => setShowSettings(!showSettings)}
                className="p-2 hover:bg-gray-100 rounded-lg"
                title="Settings"
              >
                <Settings className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>

        {/* Messages */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {messages.length === 0 && (
            <div className="text-center text-gray-500 mt-8">
              <MessageCircle className="w-12 h-12 mx-auto mb-4 opacity-50" />
              <p className="text-lg mb-2">Welcome to Vintage Story AI Assistant!</p>
              <p className="text-sm">Ask me anything about Vintage Story gameplay, crafting, or mechanics.</p>
              {!systemStatus.wiki_ready && (
                <p className="text-sm text-amber-600 mt-4">
                  <AlertCircle className="inline w-4 h-4 mr-1" />
                  Wiki knowledge base is empty. Click "Update Wiki" to download game information.
                </p>
              )}
            </div>
          )}
          
          {messages.map((message) => (
            <div
              key={message.id}
              className={`flex ${
                message.role === "user" ? "justify-end" : "justify-start"
              }`}
            >
              <div
                className={`max-w-2xl px-4 py-3 rounded-lg ${
                  message.role === "user"
                    ? "bg-blue-500 text-white"
                    : message.role === "error"
                    ? "bg-red-50 text-red-800 border border-red-200"
                    : "bg-white border shadow-sm"
                }`}
              >
                <MessageRenderer content={message.content} isUser={message.role === "user"} />
                <p className="text-xs opacity-70 mt-2">
                  {new Date(message.timestamp).toLocaleTimeString()}
                </p>
              </div>
            </div>
          ))}
          
          {isLoading && (
            <div className="flex justify-start">
              <div className="bg-white border shadow-sm rounded-lg px-4 py-3">
                <div className="flex space-x-1">
                  <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                  <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: "0.1s" }}></div>
                  <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: "0.2s" }}></div>
                </div>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input */}
        <div className="bg-white border-t p-4">
          {!systemStatus.ollama_ready && (
            <div className="mb-3 p-3 bg-amber-50 border border-amber-200 rounded-lg flex items-center space-x-2">
              <AlertCircle className="w-5 h-5 text-amber-600 flex-shrink-0" />
              <span className="text-sm text-amber-800">
                Ollama is not ready. Click "Setup Ollama" in the sidebar to install and configure it.
              </span>
            </div>
          )}
          <div className="flex space-x-2">
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyPress={(e) => e.key === "Enter" && !isLoading && sendMessage()}
              placeholder={systemStatus.ollama_ready ? "Ask about Vintage Story..." : "Ollama setup required..."}
              className="flex-1 border rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-50"
              disabled={isLoading || !systemStatus.ollama_ready}
            />
            <button
              onClick={sendMessage}
              disabled={isLoading || !inputValue.trim() || !systemStatus.ollama_ready}
              className="bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed flex items-center space-x-2"
            >
              <Send className="w-4 h-4" />
              <span>Send</span>
            </button>
          </div>
        </div>
      </div>
      
      {/* Settings Panel */}
      <SettingsPanel
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
        selectedModel={selectedModel}
        onModelChange={setSelectedModel}
        availableModels={ollamaStatus?.models || []}
        temperature={temperature}
        onTemperatureChange={setTemperature}
        maxContextChunks={maxContextChunks}
        onMaxContextChunksChange={setMaxContextChunks}
        theme={theme}
        onThemeChange={setTheme}
      />
      
      {/* Chat History */}
      <ChatHistory
        isOpen={showHistory}
        onClose={() => setShowHistory(false)}
        sessions={chatSessions}
        currentSessionId={currentSessionId}
        onSelectSession={(id) => {
          // Load session messages
          const session = chatSessions.find(s => s.id === id);
          if (session) {
            setMessages(session.messages);
            setCurrentSessionId(id);
          }
        }}
        onDeleteSession={(id) => {
          setChatSessions(prev => prev.filter(s => s.id !== id));
          if (currentSessionId === id) {
            setCurrentSessionId(null);
            setMessages([]);
          }
        }}
        onNewSession={() => {
          const newSession = {
            id: Date.now().toString(),
            title: 'New Chat',
            timestamp: new Date().toISOString(),
            messages: []
          };
          setChatSessions(prev => [...prev, newSession]);
          setCurrentSessionId(newSession.id);
          setMessages([]);
          setShowHistory(false);
        }}
        onExportSession={(id) => {
          const session = chatSessions.find(s => s.id === id);
          if (session) {
            const blob = new Blob([JSON.stringify(session, null, 2)], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `chat-${session.id}.json`;
            a.click();
            URL.revokeObjectURL(url);
          }
        }}
        onImportSession={(file) => {
          const reader = new FileReader();
          reader.onload = (e) => {
            try {
              const session = JSON.parse(e.target?.result as string);
              session.id = Date.now().toString(); // New ID for imported session
              setChatSessions(prev => [...prev, session]);
            } catch (err) {
              console.error('Failed to import session:', err);
            }
          };
          reader.readAsText(file);
        }}
      />
    </div>
  );
}

// Status Item Component
function StatusItem({ icon, label, status, isReady }: {
  icon: React.ReactNode;
  label: string;
  status: string;
  isReady: boolean;
}) {
  return (
    <div className="flex items-center justify-between p-2 rounded-lg bg-gray-50">
      <div className="flex items-center space-x-2">
        {icon}
        <span className="text-sm font-medium">{label}</span>
      </div>
      <div className="flex items-center space-x-2">
        <span className="text-xs text-gray-600">{status}</span>
        {isReady ? (
          <CheckCircle className="w-4 h-4 text-green-500" />
        ) : (
          <AlertCircle className="w-4 h-4 text-amber-500" />
        )}
      </div>
    </div>
  );
}

export default App;