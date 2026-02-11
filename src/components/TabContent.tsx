import { useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Tab } from "../hooks/useTabs";
import NewTabPage from "./NewTabPage";

interface Props {
  tab: Tab;
  onNavigate: (url: string) => void;
  onTitleChange: (title: string) => void;
}

export default function TabContent({ tab, onNavigate, onTitleChange }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);

  const getArea = useCallback(() => {
    if (!containerRef.current) return null;
    const rect = containerRef.current.getBoundingClientRect();
    return {
      x: rect.left,
      y: rect.top,
      width: rect.width,
      height: rect.height,
    };
  }, []);

  // Navigate when URL changes
  useEffect(() => {
    if (tab.url === "ostap://newtab" || !tab.url.startsWith("http")) return;

    const area = getArea();
    if (!area) return;

    invoke("navigate_tab", { url: tab.url, tabId: tab.id, area }).catch((err) => {
      console.error("navigate_tab failed:", err);
    });
  }, [tab.url, tab.id, getArea]);

  // Resize webview when container resizes
  useEffect(() => {
    if (tab.url === "ostap://newtab" || !tab.url.startsWith("http")) return;
    if (!containerRef.current) return;

    const observer = new ResizeObserver(() => {
      const area = getArea();
      if (area) {
        invoke("resize_tab", { tabId: tab.id, area }).catch(() => {});
      }
    });

    observer.observe(containerRef.current);
    return () => observer.disconnect();
  }, [tab.id, tab.url, getArea]);

  // Cleanup webview on unmount
  useEffect(() => {
    return () => {
      if (tab.url !== "ostap://newtab") {
        invoke("close_tab_webview", { tabId: tab.id }).catch(() => {});
      }
    };
  }, [tab.id]);

  if (tab.url === "ostap://newtab") {
    return <NewTabPage onNavigate={onNavigate} />;
  }

  return (
    <div ref={containerRef} className="flex-1 relative bg-bg">
      {/* The Tauri webview renders on top of this area */}
    </div>
  );
}
