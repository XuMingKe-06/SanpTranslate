# SnapTranslate

<h3 align="center">Minimal · Efficient · Privacy-First Desktop Screenshot Translation Tool</h3>

<p align="center">
  <img src="src-tauri/icons/icon.png" width="128" alt="SnapTranslate Logo" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.x-FFC131" alt="Tauri 2.x" />
  <img src="https://img.shields.io/static/v1?label=Rust&message=2024&color=orange" alt="Rust 2024" />
  <img src="https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vue.js" alt="Vue 3.5" />
  <img src="https://img.shields.io/badge/TypeScript-5.7-3178C6?logo=typescript" alt="TypeScript 5.7" />
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT License" />
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey" alt="Cross Platform" />
</p>

<p align="center">
  <a href="README.zh.md">简体中文</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.en.md">English</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a>
</p>

---

## Introduction

**SnapTranslate** is a desktop screenshot translation tool designed for developers and language learners. Select any region on the screen to instantly perform OCR text recognition + AI translation, with the translated text displayed in a right-side panel — original and translation side by side at a glance.

**Core Philosophy:** One-click region select → In-place pin → Panel translation

> Screenshots are pinned at their original position, translations shown in the right panel — no popups, no page jumps, no workflow interruption.

---

## Features

| Feature | Description |
|---------|-------------|
| **Region Screenshot Translation** | Global hotkey `Ctrl+Alt+L` activates the overlay, drag to select any region, screenshot pinned at original position automatically |
| **Clipboard Pin** | `Ctrl+Alt+P` pastes an image from the system clipboard onto the desktop for translation |
| **Text Translation** | `Ctrl+Alt+M` opens a clean text translation window with customizable target language, `Ctrl+Enter` for quick translation |
| **Local OCR** | Built-in Tesseract offline engine, supports Chinese, English, and Japanese, supports local smart auto-detection — no internet required |
| **AI Translation** | Supports any OpenAI-compatible API (bring your own model and key), directly connecting to your AI capabilities |
| **Smart Translation Cache** | Repeated content automatically matches historical records; cache hit skips the API call for instant results |
| **In-Place Pin Window** | Screenshot fixed at the original capture position, right-side translation panel supports height adjustment, transparent dark theme for distraction-free viewing |
| **Original/Translation Toggle** | One-click switch between the original screenshot text and AI translation results, convenient for side-by-side learning |
| **One-Click Copy** | Copy original text or translated text to the system clipboard |
| **Translation History** | All translation records automatically saved to local SQLite database; supports viewing, copying, deleting, and clearing |
| **Bilingual UI** | Simplified Chinese / English interface with auto-detect system language support, instant switching |
| **Privacy & Security** | Screenshots and text processed entirely locally; only translation requests communicate with your own API — **no telemetry or data upload** |
| **Auto Start** | Optional auto-start on boot, always ready when you need it |

---

## Workflow

### 1. Configure AI API

On first use, right-click the system tray icon → **Settings**, then fill in:

- **API URL**: Any OpenAI-compatible API endpoint
- **API Key**: Securely saved via the OS credential manager, never written to disk
- **Model Name**: e.g., `gpt-4o`, `deepseek-chat`, etc.
- **Target Language**: Translate to Chinese, English, Japanese, French, and 6 other languages

### 2. Common Operations

```
Press Ctrl+Alt+L         Select region, screenshot pinned at original position
                              ↓
Click "Translate" button     OCR + AI translation, results displayed in the right panel
                              ↓
Click "Copy Translation"     Copy translated text to clipboard
                              ↓
Same content next time       Auto cache hit, results shown instantly

Press Ctrl+Alt+P         Pin clipboard image to desktop for translation
Press Ctrl+Alt+M         Open text translation window, type and translate directly
```

### 3. Pin Window Operations

| Operation | Location | Description |
|-----------|----------|-------------|
| Translate | Control bar | One-click OCR + AI translation |
| Retranslate | Control bar | Skip cache, force retranslation |
| Copy Original/Translation | Control bar | One-click copy to clipboard |
| Toggle Original/Translation | Control bar | Compare before and after translation |
| Drag Window | Window title area | Freely drag (excluding button areas) |
| Stretch Translation Panel | Panel edge | Right panel supports height adjustment |
| Close | Double-click image area | Quickly close the pin window |

---

## Screenshots

> Below are previews of the application interface (the project includes a complete logo design page `logo-design.html`):

| Module | Preview |
|--------|---------|
| **Selection Overlay** | Semi-transparent dark mask + white dashed selection box + dimension indicator |
| **Pin Window** | Screenshot displayed on top at original position + bottom control bar + right translation panel |
| **Settings Page** | Naive UI dark theme, sectioned configuration: Language/General/API/Translation/Hotkeys |
| **History** | Thumbnail list + translation summary + action buttons |
| **Text Translation** | Centered on-bottom always-on-top window, clean dual-column layout |

> Run the application to experience all interfaces first-hand.

---

## Download & Installation

### Direct Download

Download the latest installer for your platform from the [Releases](https://github.com/XuMingKe-06/SanpTranslate/releases) page:

| Platform | Format |
|----------|--------|
| Windows 10+ | `.msi` / `.exe` |
| macOS 12+ | `.dmg` |
| Linux (x86\_64) | `.deb` / `.AppImage` |

### System Requirements

- **Windows**: Windows 10 (1803+), WebView2 (included with the system)
- **macOS**: macOS 12+, WebKit (included with the system), with Tesseract and language data installed via Homebrew:
  ```bash
  brew install tesseract tesseract-lang
  ```
- **Linux**: X11/Wayland support, WebKitGTK required, with Tesseract OCR engine and required language packs installed:
  - **Ubuntu / Debian**:
    ```bash
    sudo apt update
    sudo apt install tesseract-ocr tesseract-ocr-chi-sim tesseract-ocr-eng tesseract-ocr-jpn
    ```
  - **Arch Linux**:
    ```bash
    sudo pacman -S tesseract tesseract-data-chi_sim tesseract-data-eng tesseract-data-jpn
    ```

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Framework | [Tauri 2.x](https://v2.tauri.app/) |
| Frontend Framework | [Vue 3.5](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite 6](https://vitejs.dev/) |
| UI Component Library | [Naive UI](https://www.naiveui.com/) (Dark Theme) |
| Backend Language | [Rust](https://www.rust-lang.org/) (2021 edition) |
| State Management | [Pinia 3](https://pinia.vuejs.org/) |
| Routing | [Vue Router 5](https://router.vuejs.org/) |
| Internationalization | [vue-i18n 11](https://vue-i18n.intlify.dev/) |
| Screen Capture | [xcap](https://crates.io/crates/xcap) |
| OCR | Tesseract CLI (supports Chinese, English, and Japanese, supports local smart auto-detection) |
| AI Translation | HTTP (reqwest) → OpenAI-compatible API |
| Database | SQLite ([rusqlite](https://crates.io/crates/rusqlite)) |
| Secure Storage | [keyring](https://crates.io/crates/keyring) (OS credential manager) |
| Global Hotkeys | [tauri-plugin-global-shortcut](https://github.com/tauri-apps/tauri-plugin-global-shortcut) |
| Clipboard | [tauri-plugin-clipboard-manager](https://github.com/tauri-apps/tauri-plugin-clipboard-manager) |
| Auto Start | [tauri-plugin-autostart](https://github.com/tauri-apps/tauri-plugin-autostart) |

---

## Build from Source

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/) >= 1.85
- [Tauri CLI](https://v2.tauri.app/start/cli/)

### Build Steps

```bash
# 1. Clone the repository
git clone https://github.com/XuMingKe-06/SanpTranslate.git
cd SnapTranslate

# 2. Install frontend dependencies
npm install

# 3. Run in development mode (Vite HMR + Tauri)
npm run tauri dev

# 4. Production build
npm run tauri build
```

Build artifacts are located in `src-tauri/target/release/bundle/`.

---

## Configuration File Locations

| Content | Windows | macOS | Linux |
|---------|---------|-------|-------|
| Config File | `%APPDATA%\SnapTranslate\config\config.toml` | `~/Library/Application Support/SnapTranslate/config/config.toml` | `~/.config/SnapTranslate/config/config.toml` |
| History Database | `%APPDATA%\SnapTranslate\data\history.db` | `~/Library/Application Support/SnapTranslate/data/history.db` | `~/.local/share/SnapTranslate/data/history.db` |

> **API Key** is NOT stored in the configuration file; it is securely saved via the OS credential manager (Windows Credential Manager / macOS Keychain / Linux Secret Service).

---

## Project Structure

```
SnapTranslate/
├── src/                          # Frontend source (Vue 3 + TypeScript)
│   ├── components/               #   Shared components
│   │   ├── ControlBar.vue        #     Pin control bar (translate/copy/toggle)
│   │   ├── HistoryItem.vue       #     History entry
│   │   └── ShortcutInput.vue     #     Hotkey capture input
│   ├── views/                    #   Page views
│   │   ├── OverlayView.vue       #     Fullscreen selection overlay (Canvas)
│   │   ├── PinView.vue           #     Pin window (screenshot + translation panel)
│   │   ├── SettingsView.vue      #     Settings page (Naive UI)
│   │   ├── HistoryView.vue       #     History page
│   │   └── TextTranslateView.vue #     Text translation window
│   ├── stores/                   #   Pinia state management
│   │   ├── configStore.ts        #     Config state
│   │   ├── pinStore.ts           #     Pin state
│   │   └── historyStore.ts       #     History state
│   ├── i18n/                     #   Internationalization
│   │   ├── index.ts              #     vue-i18n config
│   │   └── locales/
│   │       ├── zh-CN.ts          #     Chinese language pack
│   │       └── en-US.ts          #     English language pack
│   ├── styles/                   #   Global styles
│   │   ├── variables.css         #     CSS custom properties
│   │   └── global.css            #     Global reset
│   ├── utils/                    #   Utility functions
│   │   ├── tauri.ts              #     Tauri command bindings + interface definitions
│   │   └── logger.ts             #     Structured logging
│   ├── router/
│   │   └── index.ts              #   Vue Router (5 routes)
│   └── main.ts                   #   Application entry
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── capture/mod.rs        #   Capture module (xcap wrapper)
│   │   ├── ocr/mod.rs            #   OCR module (Tesseract CLI)
│   │   ├── translate/mod.rs      #   Translation module (AI API + cache)
│   │   ├── config/               #   Configuration management (TOML + keyring)
│   │   ├── history/mod.rs        #   History (SQLite CRUD)
│   │   ├── clipboard/mod.rs      #   Clipboard read/write
│   │   ├── hotkey/mod.rs         #   Global hotkey registration
│   │   ├── window/mod.rs         #   Window management (singleton/multi-instance)
│   │   ├── tray/mod.rs           #   System tray menu
│   │   ├── commands.rs           #   21 Tauri commands
│   │   ├── error.rs              #   Unified error types
│   │   ├── lib.rs                #   Setup entry
│   │   └── main.rs               #   Main function
│   └── resources/tesseract/      #   Tesseract OCR offline data
├── docs/                         # Project documentation
│   ├── SRS.md                    #   Software Requirements Specification
│   ├── ARCHITECTURE.md           #   Architecture Design Document
│   ├── HLD.md                    #   High-Level Design
│   ├── DLD.md                    #   Detailed Design
│   ├── TEST_PLAN.md              #   Test Plan
│   └── TEST_DESIGN.md            #   Test Design Specification
├── package.json
├── CLAUDE.md                     # Development guide
└── LICENSE                       # MIT License
```

---

## Data Flow

```
[User presses global hotkey]
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                    Capture Module                          │
│  xcap fullscreen capture → cache to CachedScreenStore     │
│  → create fullscreen overlay window → user drag-select   │
│  → crop region → store_pin_image → create pin window      │
└──────────────────────────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                    Pin Window                              │
│  PinView fetches image → display screenshot + control bar │
│  User clicks "Translate" → invoke translate_image         │
└──────────────────────────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────────────────────────────┐
│                OCR + Translation Pipeline                  │
│  ① Tesseract offline recognition → text + coords          │
│  ② Check history cache ──→ Hit? ──→ Return cached result  │
│                    │                                       │
│                  Miss                                     │
│                    │                                       │
│  ③ Call AI API → Parse translation → Align coords        │
│  ④ Async save to SQLite history                           │
└──────────────────────────────────────────────────────────┘
        │
        ▼
[Translation rendered in right panel, with original/translation toggle, copy, and stretch support]
```

---

## Design Philosophy

- **Privacy First:** OCR runs locally — no risk of screenshots being uploaded to third-party services; translation communicates only with your own API endpoint; no telemetry, no tracking
- **Snap & Go:** Screenshots are pinned in place before translation — no new windows, no disruption to your current workflow
- **Offline Reliable:** Screenshot and pin functions work fully even without internet; OCR is completely offline
- **Cache Efficiency:** Repeated content matches historical cache for instant results, saving API call costs
- **Lightweight & Self-Sustaining:** Built on Tauri with small installer size and low memory footprint; Rust backend ensures high performance and low power consumption

---

## License

[MIT License](LICENSE)

Copyright © 2026 XuMingKe
