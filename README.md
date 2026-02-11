# Ostap Browser 🌐

A minimal, dark-themed web browser built with **Tauri 2.0**, **React**, and **TypeScript**.

![Ostap Browser](https://img.shields.io/badge/status-alpha-7c5cff)

## Features

- 🗂️ **Vertical sidebar tabs** — Zen Browser-inspired tab management
- 🔍 **Google search** — default search engine
- 🤖 **Jarvis AI sidebar** — collapsible AI chat assistant (placeholder)
- 🌙 **Dark minimalist design** — easy on the eyes
- ⚡ **Fast** — Tauri + Vite for instant startup

## Screenshots

> Coming soon

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri CLI](https://v2.tauri.app/start/prerequisites/)

### Install & Run

```bash
# Clone the repo
git clone https://github.com/jarvismaia97/ostap-browser.git
cd ostap-browser

# Install frontend dependencies
npm install

# Run in development mode
cargo tauri dev
```

### Build for Production

```bash
cargo tauri build
```

## Tech Stack

| Layer    | Technology              |
|----------|------------------------|
| Backend  | Tauri 2.0 (Rust)       |
| Frontend | React 19 + TypeScript  |
| Bundler  | Vite 6                 |
| Styling  | Tailwind CSS 3         |

## Project Structure

```
ostap-browser/
├── src-tauri/          # Rust backend
├── src/                # React frontend
│   ├── components/     # UI components
│   ├── hooks/          # Custom hooks (tab management)
│   └── styles/         # Theme tokens
├── package.json
└── vite.config.ts
```

## License

MIT
