# SnapTranslate

<h3 align="center">極簡 · 高效 · 隱私安全的桌面截圖翻譯工具</h3>

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

## 簡介

**SnapTranslate** 是一款面向開發者和外語學習者的桌面截圖貼圖翻譯工具。框選螢幕任意區域，瞬間完成 OCR 文字辨識 + AI 翻譯，譯文透過右側面板清晰展示，原文與譯文對照一目了然。

**核心理念：** 一鍵框選 → 原位貼圖 → 面板譯文

> 截圖貼圖在原始位置，譯文在右側面板展示 — 不彈窗、不跳轉、不打斷工作流。

---

## 功能特性

| 特性 | 說明 |
|------|------|
| **框選截圖翻譯** | 全局快速鍵 `Ctrl+Alt+L` 喚起截圖遮罩，拖拽框選任意區域，截圖自動貼在螢幕原位 |
| **剪貼簿貼圖** | `Ctrl+Alt+P` 將系統剪貼簿中的圖片貼到桌面上進行翻譯 |
| **文字翻譯** | `Ctrl+Alt+M` 開啟簡潔文字翻譯視窗，支援自訂目標語言，Ctrl+Enter 快捷翻譯 |
| **本機 OCR** | 內建 Tesseract 離線引擎，支援中簡、英、日多國語言，支援本地智慧自動語言偵測，無需連網 |
| **AI 翻譯** | 支援任意 OpenAI 相容 API（自備模型與金鑰），與你的 AI 能力直接對接 |
| **智慧翻譯快取** | 重複內容自動比對歷史記錄，命中快取則跳過 API 呼叫，秒出結果 |
| **原位貼圖視窗** | 截圖固定在原始截取位置，右側譯文面板支援高度拉伸，深色透明無干擾 |
| **原文/譯文切換** | 一鍵切換顯示截圖原文與 AI 翻譯結果，對照學習更方便 |
| **一鍵複製** | 支援複製原文、複製譯文到系統剪貼簿 |
| **翻譯歷史** | 自動保存所有翻譯記錄到本機 SQLite 資料庫，支援檢視、複製、刪除、清空 |
| **雙語介面** | 簡體中文 / English 雙語言介面，支援跟隨系統自動切換，切換即時生效 |
| **隱私安全** | 截圖和文字全在本機處理，僅翻譯請求與自配 API 通訊，**無任何遙測或資料上傳** |
| **開機自啟動** | 可選開機自動啟動，隨時待命 |

---

## 使用流程

### 1. 配置大模型 API

首次使用，右鍵系統托盤圖示 → **設定**，填入：

- **API 位址**：任意相容 OpenAI 格式的 API 端點
- **API 金鑰**：透過作業系統憑證管理員安全保存，不落碟
- **模型名稱**：例如 `gpt-4o`、`deepseek-chat` 等
- **目標語言**：翻譯成中文、英語、日語、法語等 9 種語言

### 2. 常用操作

```
按 Ctrl+Alt+L         框選截圖並貼圖到原位
                         ↓
點選貼圖下方「翻譯」按鈕    OCR + AI 翻譯，譯文在右側面板展示
                         ↓
點選「複製譯文」         將譯文複製到剪貼簿
                         ↓
下次截到相同內容         自動命中快取，即刻顯示譯文

按 Ctrl+Alt+P         將系統剪貼簿圖片貼到桌面翻譯
按 Ctrl+Alt+M         開啟文字翻譯視窗，輸入文字直接翻譯
```

### 3. 貼圖視窗操作

| 操作 | 位置 | 說明 |
|------|------|------|
| 翻譯 | 控制欄 | 一鍵 OCR + AI 翻譯 |
| 重新翻譯 | 控制欄 | 跳过快取，強制重新翻譯 |
| 複製原文/譯文 | 控制欄 | 一鍵複製到剪貼簿 |
| 原文/譯文切換 | 控制欄 | 對照檢視翻譯前後內容 |
| 拖拽視窗 | 視窗標題區 | 排除按鈕區域，可任意拖拽 |
| 拉伸譯文面板 | 面板邊緣 | 右側面板支援高度拉伸 |
| 關閉 | 雙擊圖片區域 | 快速關閉貼圖視窗 |

---

## 截圖展示

> 以下為應用程式執行介面預覽（專案配套有完整的 Logo 設計頁面 `logo-design.html`）：

| 模組 | 預覽 |
|------|------|
| **截圖遮罩** | 半透明暗色遮罩 + 白虛線框選 + 尺寸提示 |
| **貼圖視窗** | 截圖原位置置頂顯示 + 底部控制欄 + 右側譯文面板 |
| **設定頁面** | Naive UI 深色主題，分割區配置：語言/一般/API/翻譯/快速鍵 |
| **歷史記錄** | 縮圖列表 + 翻譯摘要 + 操作按鈕 |
| **文字翻譯** | 螢幕下方居中置頂視窗，簡潔雙欄佈局 |

> 可執行程式實地體驗所有介面。

---

## 下載安裝

### 直接下載

從 [Releases](https://github.com/XuMingKe-06/SanpTranslate/releases) 頁面下載對應平台的最新安裝包：

| 平台 | 格式 |
|------|------|
| Windows 10+ | `.msi` / `.exe` |
| macOS 12+ | `.dmg` |
| Linux (x86\_64) | `.deb` / `.AppImage` |

### 系統需求

- **Windows**: Windows 10 (1803+)，需 WebView2（系統內建）
- **macOS**: macOS 12+，需 WebKit（系統內建），且需透過 Homebrew 安裝 Tesseract 及語言包：
  ```bash
  brew install tesseract tesseract-lang
  ```
- **Linux**: 支援 X11/Wayland，需 WebKitGTK，且需安裝 Tesseract 引擎及對應的語言包（自動偵測和純語言模式均需對應的 `.traineddata` 檔案）：
  - **Ubuntu / Debian**:
    ```bash
    sudo apt update
    # 安裝 Tesseract 引擎及中文簡體、英文、日語語言包
    sudo apt install tesseract-ocr tesseract-ocr-chi-sim tesseract-ocr-eng tesseract-ocr-jpn
    ```
  - **Arch Linux**:
    ```bash
    sudo pacman -S tesseract tesseract-data-chi_sim tesseract-data-eng tesseract-data-jpn
    ```

---

## 技術棧

| 層級 | 技術 |
|------|------|
| 桌面框架 | [Tauri 2.x](https://v2.tauri.app/) |
| 前端框架 | [Vue 3.5](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite 6](https://vitejs.dev/) |
| UI 元件庫 | [Naive UI](https://www.naiveui.com/)（深色主題） |
| 後端語言 | [Rust](https://www.rust-lang.org/)（2021 edition） |
| 狀態管理 | [Pinia 3](https://pinia.vuejs.org/) |
| 路由 | [Vue Router 5](https://router.vuejs.org/) |
| 國際化 | [vue-i18n 11](https://vue-i18n.intlify.dev/) |
| 螢幕截圖 | [xcap](https://crates.io/crates/xcap) |
| OCR | Tesseract CLI（支援中簡、英、日等多語言，支援本地智慧自動語言偵測） |
| AI 翻譯 | HTTP (reqwest) → OpenAI 相容 API |
| 資料庫 | SQLite ([rusqlite](https://crates.io/crates/rusqlite)) |
| 安全儲存 | [keyring](https://crates.io/crates/keyring)（OS 憑證管理員） |
| 全域快速鍵 | [tauri-plugin-global-shortcut](https://github.com/tauri-apps/tauri-plugin-global-shortcut) |
| 剪貼簿 | [tauri-plugin-clipboard-manager](https://github.com/tauri-apps/tauri-plugin-clipboard-manager) |
| 開機自啟 | [tauri-plugin-autostart](https://github.com/tauri-apps/tauri-plugin-autostart) |

---

## 從原始碼建構

### 環境準備

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/) >= 1.85
- [Tauri CLI](https://v2.tauri.app/start/cli/)

### 建構步驟

```bash
# 1. 克隆倉庫
git clone https://github.com/XuMingKe-06/SanpTranslate.git
cd SnapTranslate

# 2. 安裝前端依賴
npm install

# 3. 開發模式執行（Vite HMR + Tauri）
npm run tauri dev

# 4. 生產建構
npm run tauri build
```

建構產物位於 `src-tauri/target/release/bundle/` 目錄下。

---

## 設定檔位置

| 內容 | Windows | macOS | Linux |
|------|---------|-------|-------|
| 設定檔 | `%APPDATA%\SnapTranslate\config\config.toml` | `~/Library/Application Support/SnapTranslate/config/config.toml` | `~/.config/SnapTranslate/config/config.toml` |
| 歷史資料庫 | `%APPDATA%\SnapTranslate\data\history.db` | `~/Library/Application Support/SnapTranslate/data/history.db` | `~/.local/share/SnapTranslate/data/history.db` |

> **API 金鑰**不保存在設定檔中，透過作業系統憑證管理員（Windows Credential Manager / macOS Keychain / Linux Secret Service）安全儲存。

---

## 專案結構

```
SnapTranslate/
├── src/                          # 前端原始碼 (Vue 3 + TypeScript)
│   ├── components/               #   共用元件
│   │   ├── ControlBar.vue        #     貼圖控制欄（翻譯/複製/切換）
│   │   ├── HistoryItem.vue       #     歷史記錄條目
│   │   └── ShortcutInput.vue     #     快速鍵捕獲輸入
│   ├── views/                    #   頁面檢視
│   │   ├── OverlayView.vue       #     全螢幕截圖遮罩（Canvas 框選）
│   │   ├── PinView.vue           #     貼圖視窗（截圖+譯文面板）
│   │   ├── SettingsView.vue      #     設定頁面（Naive UI）
│   │   ├── HistoryView.vue       #     歷史記錄頁面
│   │   └── TextTranslateView.vue #     文字翻譯視窗
│   ├── stores/                   #   Pinia 狀態管理
│   │   ├── configStore.ts        #     設定狀態
│   │   ├── pinStore.ts           #     貼圖狀態
│   │   └── historyStore.ts       #     歷史狀態
│   ├── i18n/                     #   國際化
│   │   ├── index.ts              #     vue-i18n 設定
│   │   └── locales/
│   │       ├── zh-CN.ts          #     簡體中文語言包
│   │       └── en-US.ts          #     英文語言包
│   ├── styles/                   #   全域樣式
│   │   ├── variables.css         #     CSS 自訂屬性
│   │   └── global.css            #     全域重設
│   ├── utils/                    #   工具函式
│   │   ├── tauri.ts              #     Tauri 命令繫結 + 介面定義
│   │   └── logger.ts             #     結構化日誌
│   ├── router/
│   │   └── index.ts              #   Vue Router（5 個路由）
│   └── main.ts                   #   應用程式進入點
├── src-tauri/                    # Rust 後端
│   ├── src/
│   │   ├── capture/mod.rs        #   截圖模組（xcap 封裝）
│   │   ├── ocr/mod.rs            #   OCR 模組（Tesseract CLI）
│   │   ├── translate/mod.rs      #   翻譯模組（AI API + 快取）
│   │   ├── config/               #   設定管理（TOML + keyring）
│   │   ├── history/mod.rs        #   歷史記錄（SQLite CRUD）
│   │   ├── clipboard/mod.rs      #   剪貼簿讀寫
│   │   ├── hotkey/mod.rs         #   全域快速鍵註冊
│   │   ├── window/mod.rs         #   視窗管理（單例/多實例）
│   │   ├── tray/mod.rs           #   系統托盤選單
│   │   ├── commands.rs           #   21 個 Tauri 命令
│   │   ├── error.rs              #   統一錯誤型別
│   │   ├── lib.rs                #   setup 進入點
│   │   └── main.rs               #   main 函式
│   └── resources/tesseract/      #   Tesseract OCR 離線資料
├── docs/                         # 專案文件
│   ├── SRS.md                    #   軟體需求規格說明書
│   ├── ARCHITECTURE.md           #   架構設計文件
│   ├── HLD.md                    #   概要設計說明書
│   ├── DLD.md                    #   詳細設計說明書
│   ├── TEST_PLAN.md              #   測試計劃
│   └── TEST_DESIGN.md            #   測試設計規格說明
├── package.json
├── CLAUDE.md                     # 開發指南
└── LICENSE                       # MIT License
```

---

## 資料流

```
[使用者按下全域快速鍵]
        │
        ▼
┌────────────────────────────────────────────────────────┐
│                    截圖模組                              │
│  xcap 全螢幕截圖 → 快取到 CachedScreenStore              │
│  → 建立全螢幕遮罩視窗 → 使用者拖拽框選 → 裁切區域        │
│  → store_pin_image → 建立貼圖視窗                       │
└────────────────────────────────────────────────────────┘
        │
        ▼
┌────────────────────────────────────────────────────────┐
│                   貼圖視窗                               │
│  PinView 拉取影像 → 顯示截圖 + 控制欄                    │
│  使用者點選「翻譯」→ invoke translate_image                │
└────────────────────────────────────────────────────────┘
        │
        ▼
┌────────────────────────────────────────────────────────┐
│               OCR + 翻譯管線                              │
│  ① Tesseract 離線辨識 → 文字 + 座標 (OcrBlock[])        │
│  ② 查詢歷史快取 ──→ 命中？──→ 直接回傳快取結果           │
│                    │                                    │
│                  未命中                                  │
│                    │                                    │
│  ③ 呼叫 AI API → 解析翻譯 → 座標對齊 → 回傳翻譯區塊     │
│  ④ 非同步儲存到 SQLite 歷史記錄                          │
└────────────────────────────────────────────────────────┘
        │
        ▼
[譯文渲染到右側面板，支援原文/譯文切換、複製、拉伸]
```

---

## 設計理念

- **隱私優先：** OCR 在本機完成，不存在截圖上傳到第三方服務的風險；翻譯僅與你自己的 API 端點通訊，無遙測、無追蹤
- **即截即用：** 截圖固定於原位再翻譯，不彈新視窗，不打斷目前操作流
- **離線可靠：** 即使網路不可用，截圖和貼圖功能完全正常，OCR 完全離線
- **快取提效：** 重複內容比對歷史快取，秒出結果，節省 API 呼叫費用
- **輕量自持：** 基於 Tauri 建構，安裝包小、記憶體佔用低，Rust 後端保證高效能和低功耗

---

## 許可

[MIT License](LICENSE)

Copyright © 2026 XuMingKe
