import { PanelLeftClose, PanelLeft, Plus } from "lucide-react";
import type { Tab } from "../hooks/useTabs";
import TabItem from "./TabItem";

interface Props {
  tabs: Tab[];
  activeTabId: string;
  expanded: boolean;
  onToggle: () => void;
  onSelectTab: (id: string) => void;
  onCloseTab: (id: string) => void;
  onNewTab: () => void;
}

export default function Sidebar({ tabs, activeTabId, expanded, onToggle, onSelectTab, onCloseTab, onNewTab }: Props) {
  return (
    <div
      className="flex flex-col bg-bg-sidebar border-r border-border h-full shrink-0 transition-all duration-200"
      style={{ width: expanded ? 240 : 48 }}
    >
      {/* Toggle + New Tab */}
      <div className="flex items-center justify-between p-2">
        <button
          onClick={onToggle}
          className="w-8 h-8 flex items-center justify-center rounded hover:bg-bg-panel text-txt-secondary transition-colors"
          title={expanded ? "Collapse sidebar" : "Expand sidebar"}
        >
          {expanded ? <PanelLeftClose size={16} /> : <PanelLeft size={16} />}
        </button>
        {expanded && (
          <button
            onClick={onNewTab}
            className="w-8 h-8 flex items-center justify-center rounded hover:bg-bg-panel text-txt-secondary transition-colors"
            title="New Tab"
          >
            <Plus size={16} />
          </button>
        )}
      </div>

      {/* Tab list */}
      <div className="flex-1 overflow-y-auto space-y-0.5 px-1">
        {tabs.map((tab) => (
          <TabItem
            key={tab.id}
            tab={tab}
            active={tab.id === activeTabId}
            expanded={expanded}
            onSelect={() => onSelectTab(tab.id)}
            onClose={() => onCloseTab(tab.id)}
          />
        ))}
      </div>

      {/* Collapsed new tab button */}
      {!expanded && (
        <div className="p-2">
          <button
            onClick={onNewTab}
            className="w-8 h-8 flex items-center justify-center rounded hover:bg-bg-panel text-txt-secondary transition-colors"
            title="New Tab"
          >
            <Plus size={16} />
          </button>
        </div>
      )}
    </div>
  );
}
