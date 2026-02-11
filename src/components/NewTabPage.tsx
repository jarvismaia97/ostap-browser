import { useState, type KeyboardEvent } from "react";
import { Search, Command } from "lucide-react";

interface Props {
  onNavigate: (url: string) => void;
}

export default function NewTabPage({ onNavigate }: Props) {
  const [query, setQuery] = useState("");

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" && query.trim()) {
      onNavigate(query.trim());
    }
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center bg-bg gap-8">
      <h1 className="text-5xl font-semibold text-txt tracking-tight">
        <span className="text-accent">Ostap</span>
      </h1>
      <p className="text-txt-secondary text-sm">A minimal browser, for the focused mind.</p>

      <div className="relative w-full max-w-lg">
        <Search size={16} className="absolute left-4 top-1/2 -translate-y-1/2 text-txt-secondary" />
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Search Google or enter URL"
          autoFocus
          className="w-full bg-bg-panel border border-border rounded-xl pl-11 pr-5 py-3 text-base text-txt placeholder:text-txt-secondary focus:border-accent focus:outline-none transition-colors"
        />
      </div>

      <div className="flex gap-6 text-txt-secondary text-xs mt-4">
        <span className="flex items-center gap-1"><Command size={11} />T New Tab</span>
        <span className="flex items-center gap-1"><Command size={11} />L Address Bar</span>
        <span className="flex items-center gap-1"><Command size={11} />W Close Tab</span>
      </div>
    </div>
  );
}
