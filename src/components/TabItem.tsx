import { X, Globe } from "lucide-react";
import type { Tab } from "../hooks/useTabs";

interface Props {
  tab: Tab;
  active: boolean;
  expanded: boolean;
  onSelect: () => void;
  onClose: () => void;
}

export default function TabItem({ tab, active, expanded, onSelect, onClose }: Props) {
  const favicon = tab.favicon || (tab.url.startsWith("http") ? `https://www.google.com/s2/favicons?domain=${new URL(tab.url).hostname}&sz=32` : null);

  return (
    <div
      onClick={onSelect}
      className={`group flex items-center gap-2 rounded cursor-pointer transition-colors ${
        active ? "bg-bg-panel text-txt" : "text-txt-secondary hover:bg-bg-panel/50"
      } ${expanded ? "px-2 py-1.5" : "p-1.5 justify-center"}`}
      title={tab.title}
    >
      {/* Favicon */}
      <div className="w-5 h-5 shrink-0 flex items-center justify-center">
        {favicon ? (
          <img src={favicon} alt="" className="w-4 h-4 rounded-sm" onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }} />
        ) : (
          <Globe size={14} className="text-txt-secondary" />
        )}
      </div>

      {/* Title + close */}
      {expanded && (
        <>
          <span className="flex-1 truncate text-sm">{tab.title}</span>
          <button
            onClick={(e) => { e.stopPropagation(); onClose(); }}
            className="opacity-0 group-hover:opacity-100 w-5 h-5 flex items-center justify-center rounded hover:bg-border text-txt-secondary transition-opacity"
          >
            <X size={12} />
          </button>
        </>
      )}
    </div>
  );
}
