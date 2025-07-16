import React from 'react';
import { X, Moon, Sun } from 'lucide-react';
import { ModelInfo } from '../types';

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  selectedModel: string;
  onModelChange: (model: string) => void;
  availableModels: ModelInfo[];
  temperature: number;
  onTemperatureChange: (temp: number) => void;
  maxContextChunks: number;
  onMaxContextChunksChange: (chunks: number) => void;
  theme: 'light' | 'dark';
  onThemeChange: (theme: 'light' | 'dark') => void;
}

export const Settings: React.FC<SettingsProps> = ({
  isOpen,
  onClose,
  selectedModel,
  onModelChange,
  availableModels,
  temperature,
  onTemperatureChange,
  maxContextChunks,
  onMaxContextChunksChange,
  theme,
  onThemeChange,
}) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl w-96 max-h-[80vh] overflow-y-auto">
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="text-lg font-semibold">Settings</h2>
          <button onClick={onClose} className="p-1 hover:bg-gray-100 rounded">
            <X className="w-5 h-5" />
          </button>
        </div>
        
        <div className="p-4 space-y-6">
          {/* Model Selection */}
          <div>
            <label className="block text-sm font-medium mb-2">Model</label>
            <select
              value={selectedModel}
              onChange={(e) => onModelChange(e.target.value)}
              className="w-full p-2 border rounded-lg"
            >
              {availableModels.map((model) => (
                <option key={model.name} value={model.name}>
                  {model.name} ({model.details.parameter_size})
                </option>
              ))}
            </select>
          </div>

          {/* Temperature */}
          <div>
            <label className="block text-sm font-medium mb-2">
              Temperature: {temperature}
            </label>
            <input
              type="range"
              min="0"
              max="2"
              step="0.1"
              value={temperature}
              onChange={(e) => onTemperatureChange(parseFloat(e.target.value))}
              className="w-full"
            />
            <div className="flex justify-between text-xs text-gray-500 mt-1">
              <span>Conservative</span>
              <span>Creative</span>
            </div>
          </div>

          {/* Max Context Chunks */}
          <div>
            <label className="block text-sm font-medium mb-2">
              Max Context Chunks: {maxContextChunks}
            </label>
            <input
              type="range"
              min="1"
              max="10"
              step="1"
              value={maxContextChunks}
              onChange={(e) => onMaxContextChunksChange(parseInt(e.target.value))}
              className="w-full"
            />
          </div>

          {/* Theme */}
          <div>
            <label className="block text-sm font-medium mb-2">Theme</label>
            <div className="flex space-x-2">
              <button
                onClick={() => onThemeChange('light')}
                className={`flex items-center space-x-2 px-3 py-2 rounded-lg border ${
                  theme === 'light' ? 'bg-blue-50 border-blue-300' : 'border-gray-300'
                }`}
              >
                <Sun className="w-4 h-4" />
                <span>Light</span>
              </button>
              <button
                onClick={() => onThemeChange('dark')}
                className={`flex items-center space-x-2 px-3 py-2 rounded-lg border ${
                  theme === 'dark' ? 'bg-blue-50 border-blue-300' : 'border-gray-300'
                }`}
              >
                <Moon className="w-4 h-4" />
                <span>Dark</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};