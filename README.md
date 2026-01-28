# JustViewer

A clean markdown viewer for macOS with Notion-style inline editing.

## Download

**[Download Latest Release](https://github.com/eatnug/JustViewer/releases/latest)**

## How to Use

1. Open JustViewer

2. Open a markdown file (Cmd+O or drag & drop)

3. Edit inline - click anywhere to start typing

4. Save changes (Cmd+S)

Or right-click any `.md` file in Finder → Open With → JustViewer

## Features

* **Notion-style Editor** - WYSIWYG markdown editing with BlockNote

* **File Association** - Open `.md`, `.markdown`, `.txt`, `.json`, `.yaml`, `.yml`, `.toml`, `.xml`, `.log`, `.ini`, `.cfg`, `.conf`, `.csv` files directly

* **Dark Mode** - Automatic system theme detection

* **Keyboard Shortcuts** - Cmd+O (open), Cmd+S (save)

* **Slash Commands** - Type `/` for quick formatting options

## Requirements

* macOS 10.15+

## Build

```bash
# Install dependencies
npm install

# Run in development
npm run tauri dev

# Build for production
npm run tauri build
```

## Tech Stack

* [Tauri](https://tauri.app/) - Rust + Web frontend

* [BlockNote](https://www.blocknotejs.org/) - Notion-style editor

* [React](https://react.dev/) - Frontend

## License

MIT
