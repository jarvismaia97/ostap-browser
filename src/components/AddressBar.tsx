import { useState, useEffect, type KeyboardEvent } from "react";
import { ArrowLeft, ArrowRight, RotateCw, Bot } from "lucide-react";

interface Props {
  url: string;
  onNavigate: (url: string) => void;
  onBack: () => void;
  onForward: () => void;
  onRefresh: () => void;
  onToggleJarvis: () => void;
  jarvisOpen: boolean;
}

export default function AddressBar({ url, onNavigate, onBack, onForward, onRefresh, onToggleJarvis, jarvisOpen }: Props) {
  const [input, setInput] = useState(url);

  useEffect(() => {
    setInput(url);
  }, [url]);

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" && input.trim()) {
      onNavigate(input.trim());
    }
  };

  return (
    <div className="flex items-center gap-2 px-3 py-2 bg-bg-sidebar border-b border-border" data-tauri-drag-region>
      {/* Nav buttons */}
      <div className="flex gap-0.5">
        <button onClick={onBack} className="w-7 h-7 flex items-center justify-center rounded hover:bg-bg-panel text-txt-secondary transition-colors">
          <ArrowLeft size={15} />
        </button>
        <button onClick={onForward} className="w-7 h-7 flex items-center justify-center rounded hover:bg-bg-panel text-txt-secondary transition-colors">
          <ArrowRight size={15} />
        </button>
        <button onClick={onRefresh} className="w-7 h-7 flex items-center justify-center rounded hover:bg-bg-panel text-txt-secondary transition-colors">
          <RotateCw size={14} />
        </button>
      </div>

      {/* URL input */}
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Search Google or enter URL"
        className="flex-1 bg-bg-panel border border-border rounded-lg px-3 py-1.5 text-sm text-txt placeholder:text-txt-secondary focus:border-accent focus:outline-none transition-colors"
      />

      {/* Jarvis toggle */}
      <button
        onClick={onToggleJarvis}
        className={`w-7 h-7 flex items-center justify-center rounded transition-colors ${
          jarvisOpen ? "bg-accent text-white" : "hover:bg-bg-panel text-txt-secondary"
        }`}
        title="Toggle Jarvis AI"
      >
        <Bot size={15} />
      </button>
    </div>
  );
}
