import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Tab } from "../hooks/useTabs";
import NewTabPage from "./NewTabPage";

interface Props {
  tab: Tab;
  onNavigate: (url: string) => void;
  onTitleChange: (title: string) => void;
}

export default function TabContent({ tab, onNavigate, onTitleChange }: Props) {
  useEffect(() => {
    if (tab.url !== "ostap://newtab" && tab.url.startsWith("http")) {
      invoke("open_url", { url: tab.url, tabId: tab.id }).catch((err) => {
        console.error("Failed to open URL:", err);
      });
    }
  }, [tab.url, tab.id]);

  if (tab.url === "ostap://newtab") {
    return <NewTabPage onNavigate={onNavigate} />;
  }

  return (
    <div className="flex-1 flex items-center justify-center bg-bg">
      <div className="text-center space-y-3">
        <div className="w-8 h-8 border-2 border-accent border-t-transparent rounded-full animate-spin mx-auto" />
        <p className="text-txt-secondary text-sm">Opening in browser window...</p>
        <p className="text-txt-secondary text-xs opacity-50">{tab.url}</p>
      </div>
    </div>
  );
}
