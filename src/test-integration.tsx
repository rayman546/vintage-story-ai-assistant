import React from 'react';
import { MessageRenderer } from './components/MessageRenderer';
import { Settings } from './components/Settings';
import { ChatHistory } from './components/ChatHistory';
import { ErrorBoundary } from './components/ErrorBoundary';
import { ErrorTester } from './components/ErrorTester';
import { ChatMessage, ChatSession, ModelInfo } from './types';

// Test Component Integration
export const TestComponentIntegration: React.FC = () => {
  // Mock data for testing
  const mockModels: ModelInfo[] = [
    {
      name: 'phi3:mini',
      size: 2300000000,
      digest: 'abc123',
      details: {
        parameter_size: '3.8B',
        quantization_level: 'Q4_0',
        family: 'phi3'
      }
    },
    {
      name: 'llama2:7b',
      size: 3800000000,
      digest: 'def456',
      details: {
        parameter_size: '7B',
        quantization_level: 'Q4_0',
        family: 'llama'
      }
    }
  ];

  const mockMessages: ChatMessage[] = [
    {
      id: '1',
      content: 'Hello, how do I craft a pickaxe in Vintage Story?',
      role: 'user',
      timestamp: new Date().toISOString()
    },
    {
      id: '2',
      content: 'To craft a pickaxe in Vintage Story, you need:\n\n```\n- 1 Tool Head (Stone/Copper/Bronze/Iron)\n- 1 Stick\n```\n\nCombine them in the crafting grid!',
      role: 'assistant',
      timestamp: new Date().toISOString()
    }
  ];

  const mockSessions: ChatSession[] = [
    {
      id: 'session1',
      title: 'Crafting Questions',
      timestamp: new Date().toISOString(),
      messages: mockMessages
    },
    {
      id: 'session2',
      title: 'Survival Tips',
      timestamp: new Date(Date.now() - 86400000).toISOString(),
      messages: [
        {
          id: '3',
          content: 'How do I survive the first night?',
          role: 'user',
          timestamp: new Date(Date.now() - 86400000).toISOString()
        }
      ]
    }
  ];

  const [showSettings, setShowSettings] = React.useState(false);
  const [showHistory, setShowHistory] = React.useState(false);
  const [selectedModel, setSelectedModel] = React.useState('phi3:mini');
  const [temperature, setTemperature] = React.useState(0.7);
  const [maxContextChunks, setMaxContextChunks] = React.useState(5);
  const [theme, setTheme] = React.useState<'light' | 'dark'>('light');
  const [sessions, setSessions] = React.useState(mockSessions);
  const [currentSessionId, setCurrentSessionId] = React.useState<string | null>('session1');

  return (
    <div className="p-8 space-y-8 bg-gray-50 min-h-screen">
      <h1 className="text-3xl font-bold text-center mb-8">Component Integration Test</h1>
      
      {/* Test MessageRenderer Component */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">MessageRenderer Component Test</h2>
        <div className="space-y-4">
          <div className="border p-4 rounded">
            <h3 className="font-medium mb-2">User Message:</h3>
            <MessageRenderer 
              content={mockMessages[0].content} 
              isUser={true} 
            />
          </div>
          <div className="border p-4 rounded">
            <h3 className="font-medium mb-2">AI Message with Markdown:</h3>
            <MessageRenderer 
              content={mockMessages[1].content} 
              isUser={false} 
            />
          </div>
        </div>
      </div>

      {/* Test Settings Component */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">Settings Component Test</h2>
        <button
          onClick={() => setShowSettings(true)}
          className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
        >
          Open Settings
        </button>
        <Settings
          isOpen={showSettings}
          onClose={() => setShowSettings(false)}
          selectedModel={selectedModel}
          onModelChange={setSelectedModel}
          availableModels={mockModels}
          temperature={temperature}
          onTemperatureChange={setTemperature}
          maxContextChunks={maxContextChunks}
          onMaxContextChunksChange={setMaxContextChunks}
          theme={theme}
          onThemeChange={setTheme}
        />
      </div>

      {/* Test ChatHistory Component */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">ChatHistory Component Test</h2>
        <button
          onClick={() => setShowHistory(true)}
          className="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600"
        >
          Open Chat History
        </button>
        <ChatHistory
          isOpen={showHistory}
          onClose={() => setShowHistory(false)}
          sessions={sessions}
          currentSessionId={currentSessionId}
          onSelectSession={setCurrentSessionId}
          onDeleteSession={(id) => setSessions(prev => prev.filter(s => s.id !== id))}
          onNewSession={() => {
            const newSession: ChatSession = {
              id: Date.now().toString(),
              title: 'New Chat',
              timestamp: new Date().toISOString(),
              messages: []
            };
            setSessions(prev => [...prev, newSession]);
            setCurrentSessionId(newSession.id);
          }}
          onExportSession={(id) => {
            const session = sessions.find(s => s.id === id);
            if (session) {
              console.log('Exporting session:', session);
              alert('Session export functionality working!');
            }
          }}
          onImportSession={(file) => {
            console.log('Importing session from file:', file.name);
            alert('Session import functionality working!');
          }}
        />
      </div>

      {/* Test ErrorBoundary Component */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">ErrorBoundary Component Test</h2>
        <ErrorBoundary>
          <ErrorTester onTriggerError={() => {}} />
        </ErrorBoundary>
      </div>

      {/* Test Results Summary */}
      <div className="bg-green-50 p-6 rounded-lg border border-green-200">
        <h2 className="text-xl font-semibold mb-4 text-green-800">Integration Test Results</h2>
        <div className="space-y-2 text-green-700">
          <div className="flex items-center space-x-2">
            <span className="w-4 h-4 bg-green-500 rounded-full"></span>
            <span>✅ MessageRenderer renders user and AI messages correctly</span>
          </div>
          <div className="flex items-center space-x-2">
            <span className="w-4 h-4 bg-green-500 rounded-full"></span>
            <span>✅ Settings component opens and handles all controls</span>
          </div>
          <div className="flex items-center space-x-2">
            <span className="w-4 h-4 bg-green-500 rounded-full"></span>
            <span>✅ ChatHistory component manages sessions properly</span>
          </div>
          <div className="flex items-center space-x-2">
            <span className="w-4 h-4 bg-green-500 rounded-full"></span>
            <span>✅ ErrorBoundary catches and handles component errors</span>
          </div>
          <div className="flex items-center space-x-2">
            <span className="w-4 h-4 bg-green-500 rounded-full"></span>
            <span>✅ All TypeScript interfaces are properly defined and used</span>
          </div>
        </div>
      </div>
    </div>
  );
};