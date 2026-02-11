import { useReducer, useCallback } from "react";

export interface Tab {
  id: string;
  title: string;
  url: string;
  favicon?: string;
}

interface TabState {
  tabs: Tab[];
  activeTabId: string;
}

type TabAction =
  | { type: "ADD_TAB"; tab?: Partial<Tab> }
  | { type: "CLOSE_TAB"; id: string }
  | { type: "SET_ACTIVE"; id: string }
  | { type: "UPDATE_TAB"; id: string; updates: Partial<Tab> }
  | { type: "REORDER"; fromIndex: number; toIndex: number };

let nextId = 1;
function makeId() {
  return `tab-${nextId++}`;
}

function newTab(overrides?: Partial<Tab>): Tab {
  return {
    id: makeId(),
    title: "New Tab",
    url: "ostap://newtab",
    ...overrides,
  };
}

function reducer(state: TabState, action: TabAction): TabState {
  switch (action.type) {
    case "ADD_TAB": {
      const tab = newTab(action.tab);
      return { tabs: [...state.tabs, tab], activeTabId: tab.id };
    }
    case "CLOSE_TAB": {
      const remaining = state.tabs.filter((t) => t.id !== action.id);
      if (remaining.length === 0) {
        const tab = newTab();
        return { tabs: [tab], activeTabId: tab.id };
      }
      const activeStillExists = remaining.some((t) => t.id === state.activeTabId);
      return {
        tabs: remaining,
        activeTabId: activeStillExists
          ? state.activeTabId
          : remaining[remaining.length - 1].id,
      };
    }
    case "SET_ACTIVE":
      return { ...state, activeTabId: action.id };
    case "UPDATE_TAB":
      return {
        ...state,
        tabs: state.tabs.map((t) =>
          t.id === action.id ? { ...t, ...action.updates } : t
        ),
      };
    case "REORDER": {
      const tabs = [...state.tabs];
      const [moved] = tabs.splice(action.fromIndex, 1);
      tabs.splice(action.toIndex, 0, moved);
      return { ...state, tabs };
    }
    default:
      return state;
  }
}

const initialTab = newTab();
const initialState: TabState = {
  tabs: [initialTab],
  activeTabId: initialTab.id,
};

export function useTabs() {
  const [state, dispatch] = useReducer(reducer, initialState);

  const addTab = useCallback((tab?: Partial<Tab>) => dispatch({ type: "ADD_TAB", tab }), []);
  const closeTab = useCallback((id: string) => dispatch({ type: "CLOSE_TAB", id }), []);
  const setActiveTab = useCallback((id: string) => dispatch({ type: "SET_ACTIVE", id }), []);
  const updateTab = useCallback(
    (id: string, updates: Partial<Tab>) => dispatch({ type: "UPDATE_TAB", id, updates }),
    []
  );
  const reorderTab = useCallback(
    (fromIndex: number, toIndex: number) => dispatch({ type: "REORDER", fromIndex, toIndex }),
    []
  );

  const activeTab = state.tabs.find((t) => t.id === state.activeTabId) ?? state.tabs[0];

  return {
    tabs: state.tabs,
    activeTab,
    activeTabId: state.activeTabId,
    addTab,
    closeTab,
    setActiveTab,
    updateTab,
    reorderTab,
  };
}
