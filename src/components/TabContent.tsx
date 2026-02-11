import { useEffect, useRef, useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Loader2 } from "lucide-react";
import type { Tab } from "../hooks/useTabs";
import NewTabPage from "./NewTabPage";

interface TabUpdate {
  tab_id: string;
  url: string;
  title: string;
}

interface Props {
  tab: Tab;
  onNavigate: (url: string) => void;
  onTitleChange: (title: string) => void;
  onUrlChange: (url: string) => void;
}

export default function TabContent({ tab, onNavigate, onTitleChange, onUrlChange }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [loading, setLoading] = useState(false);

  const getArea = useCallback(() => {
    if (!containerRef.current) return null;
    const rect = containerRef.current.getBoundingClientRect();
    // Note: Tauri positions child webviews relative to the window content area
    // which starts below the titlebar. getBoundingClientRect is relative to the
    // main webview viewport, which already excludes the titlebar.
    return {
      x: Math.round(rect.left),
      y: Math.round(rect.top),
      width: Math.round(rect.width),
      height: Math.round(rect.height),
    };
  }, []);

  // Listen for tab updates from Rust
  useEffect(() => {
    const unlisten = listen<TabUpdate>("tab-updated", (event) => {
      if (event.payload.tab_id === tab.id) {
        if (event.payload.url) onUrlChange(event.payload.url);
        if (event.payload.title) onTitleChange(event.payload.title);
        setLoading(false);
      }
    });

    return () => { unlisten.then(fn => fn()); };
  }, [tab.id, onTitleChange, onUrlChange]);

  // Navigate when URL changes
  useEffect(() => {
    if (tab.url === "ostap://newtab" || !tab.url.startsWith("http")) {
      invoke("hide_all_tabs").catch(() => {});
      return;
    }

    setLoading(true);
    const timer = setTimeout(() => {
      const area = getArea();
      if (!area) return;
      console.log("navigate_tab area:", JSON.stringify(area));
      invoke("navigate_tab", { url: tab.url, tabId: tab.id, area })
        .then(() => console.log("navigate_tab OK"))
        .catch((err) => {
          console.error("navigate_tab failed:", err);
          setLoading(false);
        });
    }, 30);

    return () => clearTimeout(timer);
  }, [tab.url, tab.id, getArea]);

  // Resize webview when container resizes
  useEffect(() => {
    if (tab.url === "ostap://newtab" || !tab.url.startsWith("http")) return;
    if (!containerRef.current) return;

    const observer = new ResizeObserver(() => {
      const area = getArea();
      if (area) invoke("resize_tab", { tabId: tab.id, area }).catch(() => {});
    });

    observer.observe(containerRef.current);
    return () => observer.disconnect();
  }, [tab.id, tab.url, getArea]);

  if (tab.url === "ostap://newtab") {
    return <NewTabPage onNavigate={onNavigate} />;
  }

  return (
    <div ref={containerRef} className="flex-1 relative bg-bg">
      {loading && (
        <div className="absolute inset-0 flex items-center justify-center bg-bg/80 z-10 backdrop-blur-sm transition-opacity">
          <Loader2 size={20} className="text-accent animate-spin" />
        </div>
      )}
    </div>
  );
}
