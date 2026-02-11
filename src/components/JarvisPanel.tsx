import { useState } from "react";

interface Props {
  onClose: () => void;
}

export default function JarvisPanel({ onClose }: Props) {
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState<{ role: "user" | "assistant"; text: string }[]>([
    { role: "assistant", text: "Hey! I'm Jarvis, your AI assistant. How can I help?" },
  ]);

  const handleSend = () => {
    if (!input.trim()) return;
    setMessages((prev) => [
      ...prev,
      { role: "user", text: input },
      { role: "assistant", text: "This is a placeholder response. AI integration coming soon!" },
    ]);
    setInput("");
  };

  return (
    <div className="w-80 bg-bg-sidebar border-l border-border flex flex-col h-full shrink-0">
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 border-b border-border">
        <span className="text-sm font-medium text-txt">🤖 Jarvis AI</span>
        <button onClick={onClose} className="text-txt-secondary hover:text-txt text-sm">✕</button>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-3 space-y-3">
        {messages.map((msg, i) => (
          <div key={i} className={`text-sm ${msg.role === "user" ? "text-right" : ""}`}>
            <div
              className={`inline-block px-3 py-2 rounded-lg max-w-[90%] ${
                msg.role === "user"
                  ? "bg-accent text-white"
                  : "bg-bg-panel text-txt"
              }`}
            >
              {msg.text}
            </div>
          </div>
        ))}
      </div>

      {/* Input */}
      <div className="p-3 border-t border-border">
        <div className="flex gap-2">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSend()}
            placeholder="Ask Jarvis..."
            className="flex-1 bg-bg-panel border border-border rounded-lg px-3 py-1.5 text-sm text-txt placeholder:text-txt-secondary focus:border-accent"
          />
          <button onClick={handleSend} className="px-3 py-1.5 bg-accent text-white rounded-lg text-sm hover:opacity-90">
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
