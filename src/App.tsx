import { useState } from "react";
import { useTabs } from "./hooks/useTabs";
import Sidebar from "./components/Sidebar";
import AddressBar from "./components/AddressBar";
import TabContent from "./components/TabContent";
import JarvisPanel from "./components/JarvisPanel";

function App() {
  const { tabs, activeTab, activeTabId, addTab, closeTab, setActiveTab, updateTab } = useTabs();
  const [sidebarExpanded, setSidebarExpanded] = useState(true);
  const [jarvisOpen, setJarvisOpen] = useState(false);

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

  return (
    <div className="flex h-screen w-screen bg-bg overflow-hidden">
      <Sidebar
        tabs={tabs}
        activeTabId={activeTabId}
        expanded={sidebarExpanded}
        onToggle={() => setSidebarExpanded(!sidebarExpanded)}
        onSelectTab={setActiveTab}
        onCloseTab={closeTab}
        onNewTab={() => addTab()}
      />

      <div className="flex flex-col flex-1 min-w-0">
        <AddressBar
          url={activeTab.url.startsWith("ostap://") ? "" : activeTab.url}
          onNavigate={handleNavigate}
          onToggleJarvis={() => setJarvisOpen(!jarvisOpen)}
          jarvisOpen={jarvisOpen}
        />
        <TabContent tab={activeTab} onTitleChange={(title) => updateTab(activeTabId, { title })} />
      </div>

      {jarvisOpen && <JarvisPanel onClose={() => setJarvisOpen(false)} />}
    </div>
  );
}

export default App;
