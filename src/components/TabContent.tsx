import type { Tab } from "../hooks/useTabs";
import NewTabPage from "./NewTabPage";

interface Props {
  tab: Tab;
  onTitleChange: (title: string) => void;
}

export default function TabContent({ tab, onTitleChange }: Props) {
  if (tab.url === "ostap://newtab") {
    return <NewTabPage onNavigate={(url) => onTitleChange(url)} />;
  }

  return (
    <div className="flex-1 bg-white relative">
      <iframe
        key={tab.id + tab.url}
        src={tab.url}
        className="w-full h-full border-none"
        sandbox="allow-same-origin allow-scripts allow-popups allow-forms allow-presentation"
        title={tab.title}
        onLoad={(e) => {
          try {
            const iframe = e.target as HTMLIFrameElement;
            const title = iframe.contentDocument?.title;
            if (title) onTitleChange(title);
          } catch {
            // Cross-origin, can't access title
          }
        }}
      />
    </div>
  );
}
