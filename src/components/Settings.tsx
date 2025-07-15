import React from 'react';
import { X, Moon, Sun, HardDrive, Globe } from 'lucide-react';

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  selectedModel: string;
  onModelChange: (model: string) => void;
  availableModels: Array<{ name: string }>;
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
      <div className="bg-white rounded-lg shadow-xl w-full max-w-md p-6">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-semibold">Settings</h2>
          <button
            onClick={onClose}
            className="p-1 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="space-y-6">
          {/* Model Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              <Globe className="inline w-4 h-4 mr-1" />
              Language Model
            </label>
            <select
              value={selectedModel}
              onChange={(e) => onModelChange(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {availableModels.map((model) => (
                <option key={model.name} value={model.name}>
                  {model.name}
                </option>
              ))}
            </select>
          </div>

          {/* Temperature */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Temperature: {temperature.toFixed(1)}
            </label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.1"
              value={temperature}
              onChange={(e) => onTemperatureChange(parseFloat(e.target.value))}
              className="w-full"
            />
            <div className="flex justify-between text-xs text-gray-500 mt-1">
              <span>Precise</span>
              <span>Creative</span>
            </div>
          </div>

          {/* Context Chunks */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Context Chunks: {maxContextChunks}
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
            <div className="flex justify-between text-xs text-gray-500 mt-1">
              <span>Less Context</span>
              <span>More Context</span>
            </div>
          </div>

          {/* Theme */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Theme
            </label>
            <div className="flex space-x-2">
              <button
                onClick={() => onThemeChange('light')}
                className={`flex-1 flex items-center justify-center py-2 px-4 rounded-lg border transition-colors ${
                  theme === 'light'
                    ? 'bg-blue-50 border-blue-500 text-blue-700'
                    : 'bg-gray-50 border-gray-300 text-gray-700 hover:bg-gray-100'
                }`}
              >
                <Sun className="w-4 h-4 mr-2" />
                Light
              </button>
              <button
                onClick={() => onThemeChange('dark')}
                className={`flex-1 flex items-center justify-center py-2 px-4 rounded-lg border transition-colors ${
                  theme === 'dark'
                    ? 'bg-blue-50 border-blue-500 text-blue-700'
                    : 'bg-gray-50 border-gray-300 text-gray-700 hover:bg-gray-100'
                }`}
              >
                <Moon className="w-4 h-4 mr-2" />
                Dark
              </button>
            </div>
          </div>

          {/* Storage Info */}
          <div className="pt-4 border-t">
            <div className="flex items-center text-sm text-gray-600">
              <HardDrive className="w-4 h-4 mr-2" />
              <span>All data is stored locally on your device</span>
            </div>
          </div>
        </div>

        <div className="mt-6 flex justify-end space-x-3">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={onClose}
            className="px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 rounded-lg transition-colors"
          >
            Save
          </button>
        </div>
      </div>
    </div>
  );
};
