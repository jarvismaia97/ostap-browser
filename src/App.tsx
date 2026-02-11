import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useTabs } from "./hooks/useTabs";
import Sidebar from "./components/Sidebar";
import AddressBar from "./components/AddressBar";
import TabContent from "./components/TabContent";
import JarvisPanel from "./components/JarvisPanel";
import Onboarding from "./components/Onboarding";

const ONBOARDING_KEY = "ostap-onboarding-done";

function App() {
  const { tabs, activeTab, activeTabId, addTab, closeTab, setActiveTab, updateTab } = useTabs();
  const [sidebarExpanded, setSidebarExpanded] = useState(true);
  const [jarvisOpen, setJarvisOpen] = useState(false);
  const [showOnboarding, setShowOnboarding] = useState(false);

  useEffect(() => {
    if (!localStorage.getItem(ONBOARDING_KEY)) {
      setShowOnboarding(true);
    }
  }, []);

  // Listen for global shortcuts from Tauri (works even when child webview has focus)
  useEffect(() => {
    const unlisten = listen<string>("shortcut", (event) => {
      if (event.payload === "new-tab") addTab();
      else if (event.payload === "close-tab") handleCloseTab(activeTabId);
    });
    return () => { unlisten.then(fn => fn()); };
  }, [activeTabId, addTab]);

  const completeOnboarding = () => {
    localStorage.setItem(ONBOARDING_KEY, "true");
    setShowOnboarding(false);
  };

  const handleNavigate = (url: string) => {
    let finalUrl = url;
    if (!url.startsWith("http://") && !url.startsWith("https://") && !url.startsWith("ostap://")) {
      if (url.includes(".") && !url.includes(" ")) {
        finalUrl = `https://${url}`;
      } else {
        finalUrl = `https://www.google.com/search?q=${encodeURIComponent(url)}`;
      }
    }
    updateTab(activeTabId, { url: finalUrl, title: finalUrl });
  };

  const handleCloseTab = (id: string) => {
    // Close the webview in Tauri
    invoke("close_tab_webview", { tabId: id }).catch(() => {});
    closeTab(id);
  };

  return (
    <div className="flex h-screen w-screen bg-bg overflow-hidden">
      {showOnboarding && <Onboarding onComplete={completeOnboarding} />}

      <Sidebar
        tabs={tabs}
        activeTabId={activeTabId}
        expanded={sidebarExpanded}
        onToggle={() => setSidebarExpanded(!sidebarExpanded)}
        onSelectTab={setActiveTab}
        onCloseTab={handleCloseTab}
        onNewTab={() => addTab()}
      />

      <div className="flex flex-col flex-1 min-w-0">
        <AddressBar
          url={activeTab.url.startsWith("ostap://") ? "" : activeTab.url}
          onNavigate={handleNavigate}
          onBack={() => {}}
          onForward={() => {}}
          onRefresh={() => {
            const url = activeTab.url;
            updateTab(activeTabId, { url: "ostap://newtab" });
            setTimeout(() => updateTab(activeTabId, { url }), 50);
          }}
          onToggleJarvis={() => setJarvisOpen(!jarvisOpen)}
          jarvisOpen={jarvisOpen}
        />
        <TabContent
          tab={activeTab}
          allTabs={tabs}
          onNavigate={handleNavigate}
          onTitleChange={(title) => updateTab(activeTabId, { title })}
          onUrlChange={(url) => updateTab(activeTabId, { url })}
        />
      </div>

      {jarvisOpen && <JarvisPanel onClose={() => setJarvisOpen(false)} />}
    </div>
  );
}

export default App;
