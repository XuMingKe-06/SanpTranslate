# SnapTranslate

<h3 align="center">极简 · 高效 · 隐私安全的桌面截屏翻译工具</h3>

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
  <a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a>
</p>

---

## 简介

**SnapTranslate** 是一款面向开发者和外语学习者的桌面截屏贴图翻译工具。框选屏幕任意区域，瞬间完成 OCR 文字识别 + AI 翻译，译文通过右侧面板清晰展示，原文与译文对照一目了然。

**核心理念：** 一键框选 → 原位贴图 → 面板译文

> 截图贴图在原始位置，译文在右侧面板展示 — 不弹窗、不跳转、不打断工作流。

---

## 功能特性

| 特性 | 说明 |
|------|------|
| **框选截图翻译** | 全局快捷键 `Ctrl+Alt+L` 唤起截图蒙版，拖拽框选任意区域，截图自动贴在屏幕原位 |
| **剪贴板贴图** | `Ctrl+Alt+P` 将系统剪贴板中的图片贴到桌面上进行翻译 |
| **文本翻译** | `Ctrl+Alt+M` 打开简洁文本翻译窗口，支持自定义目标语言，Ctrl+Enter 快捷翻译 |
| **本地 OCR** | 内置 Tesseract 离线引擎，支持中简、英、日多国语言，支持本地智能自动语言检测，无需联网 |
| **AI 翻译** | 支持任意 OpenAI 兼容 API（自备模型与密钥），与你的 AI 能力直接对接 |
| **智能翻译缓存** | 重复内容自动匹配历史记录，命中缓存则跳过 API 调用，秒出结果 |
| **原位贴图窗口** | 截图固定在原始截取位置，右侧译文面板支持高度拉伸，深色透明无干扰 |
| **原文/译文切换** | 一键切换显示截图原文与 AI 翻译结果，对照学习更方便 |
| **一键复制** | 支持复制原文、复制译文到系统剪贴板 |
| **翻译历史** | 自动保存所有翻译记录到本地 SQLite 数据库，支持查看、复制、删除、清空 |
| **双语界面** | 简体中文 / English 双语言界面，支持跟随系统自动切换，切换即时生效 |
| **隐私安全** | 截图和文字全在本地处理，仅翻译请求与自配 API 通信，**无任何遥测或数据上传** |
| **开机自启动** | 可选开机自动启动，随时待命 |

---

## 使用流程

### 1. 配置大模型 API

首次使用，右键系统托盘图标 → **设置**，填入：

- **API 地址**：任意兼容 OpenAI 格式的 API 端点
- **API 密钥**：通过操作系统凭据管理器安全保存，不落盘
- **模型名称**：例如 `gpt-4o`、`deepseek-chat` 等
- **目标语言**：翻译成中文、英语、日语、法语等 9 种语言

### 2. 常用操作

```
按 Ctrl+Alt+L         框选截图并贴图到原位
                         ↓
点击贴图下方「翻译」按钮    OCR + AI 翻译，译文在右侧面板展示
                         ↓
点击「复制译文」         将译文复制到剪贴板
                         ↓
下次截到相同内容         自动命中缓存，即刻显示译文

按 Ctrl+Alt+P         将系统剪贴板图片贴到桌面翻译
按 Ctrl+Alt+M         打开文本翻译窗口，输入文本直接翻译
```

### 3. 贴图窗口操作

| 操作 | 位置 | 说明 |
|------|------|------|
| 翻译 | 控制栏 | 一键 OCR + AI 翻译 |
| 重新翻译 | 控制栏 | 跳过缓存，强制重新翻译 |
| 复制原文/译文 | 控制栏 | 一键复制到剪贴板 |
| 原文/译文切换 | 控制栏 | 对照查看翻译前后内容 |
| 拖拽窗口 | 窗口标题区 | 排除按钮区域，可任意拖拽 |
| 拉伸译文面板 | 面板边缘 | 右侧面板支持高度拉伸 |
| 关闭 | 双击图片区域 | 快速关闭贴图窗口 |

---

## 截图展示

> 以下为应用运行界面预览（项目配套有完整的 logo 设计页面 `logo-design.html`）：

| 模块 | 预览 |
|------|------|
| **截图蒙版** | 半透明暗色遮罩 + 白虚线框选 + 尺寸提示 |
| **贴图窗口** | 截图原位置置顶显示 + 底部控制栏 + 右侧译文面板 |
| **设置页面** | Naive UI 深色主题，分区域配置：语言/通用/API/翻译/快捷键 |
| **历史记录** | 缩略图列表 + 翻译摘要 + 操作按钮 |
| **文本翻译** | 屏幕下方居中置顶窗口，简洁双栏布局 |

> 可运行应用实地体验所有界面。

---

## 下载安装

### 直接下载

从 [Releases](https://github.com/XuMingKe-06/SanpTranslate/releases) 页面下载对应平台的最新安装包：

| 平台 | 格式 |
|------|------|
| Windows 10+ | `.msi` / `.exe` |
| macOS 12+ | `.dmg` |
| Linux (x86\_64) | `.deb` / `.AppImage` |

### 系统要求

- **Windows**: Windows 10 (1803+)，需 WebView2（系统自带）
- **macOS**: macOS 12+，需 WebKit（系统自带），且需通过 Homebrew 安装 Tesseract 及语言包：
  ```bash
  brew install tesseract tesseract-lang
  ```
- **Linux**: 支持 X11/Wayland，需 WebKitGTK，且需安装 Tesseract 引擎及对应的语言包（自动检测和纯语言模式均需对应的 `.traineddata` 文件）：
  - **Ubuntu / Debian**:
    ```bash
    sudo apt update
    # 安装 Tesseract 引擎及中文简体、英文、日语语言包
    sudo apt install tesseract-ocr tesseract-ocr-chi-sim tesseract-ocr-eng tesseract-ocr-jpn
    ```
  - **Arch Linux**:
    ```bash
    sudo pacman -S tesseract tesseract-data-chi_sim tesseract-data-eng tesseract-data-jpn
    ```

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | [Tauri 2.x](https://v2.tauri.app/) |
| 前端框架 | [Vue 3.5](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite 6](https://vitejs.dev/) |
| UI 组件库 | [Naive UI](https://www.naiveui.com/)（深色主题） |
| 后端语言 | [Rust](https://www.rust-lang.org/)（2021 edition） |
| 状态管理 | [Pinia 3](https://pinia.vuejs.org/) |
| 路由 | [Vue Router 5](https://router.vuejs.org/) |
| 国际化 | [vue-i18n 11](https://vue-i18n.intlify.dev/) |
| 屏幕截图 | [xcap](https://crates.io/crates/xcap) |
| OCR | Tesseract CLI（支持中简、英、日等多语言，支持本地智能自动语言检测） |
| AI 翻译 | HTTP (reqwest) → OpenAI 兼容 API |
| 数据库 | SQLite ([rusqlite](https://crates.io/crates/rusqlite)) |
| 安全存储 | [keyring](https://crates.io/crates/keyring)（OS 凭据管理器） |
| 全局快捷键 | [tauri-plugin-global-shortcut](https://github.com/tauri-apps/tauri-plugin-global-shortcut) |
| 剪贴板 | [tauri-plugin-clipboard-manager](https://github.com/tauri-apps/tauri-plugin-clipboard-manager) |
| 开机自启 | [tauri-plugin-autostart](https://github.com/tauri-apps/tauri-plugin-autostart) |

---

## 从源码构建

### 环境准备

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/) >= 1.85
- [Tauri CLI](https://v2.tauri.app/start/cli/)

### 构建步骤

```bash
# 1. 克隆仓库
git clone https://github.com/XuMingKe-06/SanpTranslate.git
cd SnapTranslate

# 2. 安装前端依赖
npm install

# 3. 开发模式运行（Vite HMR + Tauri）
npm run tauri dev

# 4. 生产构建
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录下。

---

## 配置文件位置

| 内容 | Windows | macOS | Linux |
|------|---------|-------|-------|
| 配置文件 | `%APPDATA%\SnapTranslate\config\config.toml` | `~/Library/Application Support/SnapTranslate/config/config.toml` | `~/.config/SnapTranslate/config/config.toml` |
| 历史数据库 | `%APPDATA%\SnapTranslate\data\history.db` | `~/Library/Application Support/SnapTranslate/data/history.db` | `~/.local/share/SnapTranslate/data/history.db` |

> **API 密钥**不保存在配置文件中，通过操作系统凭据管理器（Windows Credential Manager / macOS Keychain / Linux Secret Service）安全存储。

---

## 项目结构

```
SnapTranslate/
├── src/                          # 前端源码 (Vue 3 + TypeScript)
│   ├── components/               #   公共组件
│   │   ├── ControlBar.vue        #     贴图控制栏（翻译/复制/切换）
│   │   ├── HistoryItem.vue       #     历史记录条目
│   │   └── ShortcutInput.vue     #     快捷键捕获输入
│   ├── views/                    #   页面视图
│   │   ├── OverlayView.vue       #     全屏截图蒙版（Canvas 框选）
│   │   ├── PinView.vue           #     贴图窗口（截图+译文面板）
│   │   ├── SettingsView.vue      #     设置页面（Naive UI）
│   │   ├── HistoryView.vue       #     历史记录页面
│   │   └── TextTranslateView.vue #     文本翻译窗口
│   ├── stores/                   #   Pinia 状态管理
│   │   ├── configStore.ts        #     配置状态
│   │   ├── pinStore.ts           #     贴图状态
│   │   └── historyStore.ts       #     历史状态
│   ├── i18n/                     #   国际化
│   │   ├── index.ts              #     vue-i18n 配置
│   │   └── locales/
│   │       ├── zh-CN.ts          #     中文语言包
│   │       └── en-US.ts          #     英文语言包
│   ├── styles/                   #   全局样式
│   │   ├── variables.css         #     CSS 自定义属性
│   │   └── global.css            #     全局重置
│   ├── utils/                    #   工具函数
│   │   ├── tauri.ts              #     Tauri 命令绑定 + 接口定义
│   │   └── logger.ts             #     结构化日志
│   ├── router/
│   │   └── index.ts              #   Vue Router（5 个路由）
│   └── main.ts                   #   应用入口
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── capture/mod.rs        #   截图模块（xcap 封装）
│   │   ├── ocr/mod.rs            #   OCR 模块（Tesseract CLI）
│   │   ├── translate/mod.rs      #   翻译模块（AI API + 缓存）
│   │   ├── config/               #   配置管理（TOML + keyring）
│   │   ├── history/mod.rs        #   历史记录（SQLite CRUD）
│   │   ├── clipboard/mod.rs      #   剪贴板读写
│   │   ├── hotkey/mod.rs         #   全局快捷键注册
│   │   ├── window/mod.rs         #   窗口管理（单例/多实例）
│   │   ├── tray/mod.rs           #   系统托盘菜单
│   │   ├── commands.rs           #   21 个 Tauri 命令
│   │   ├── error.rs              #   统一错误类型
│   │   ├── lib.rs                #   setup 入口
│   │   └── main.rs               #   main 函数
│   └── resources/tesseract/      #   Tesseract OCR 离线数据
├── docs/                         # 项目文档
│   ├── SRS.md                    #   软件需求规格说明书
│   ├── ARCHITECTURE.md           #   架构设计文档
│   ├── HLD.md                    #   概要设计说明书
│   ├── DLD.md                    #   详细设计说明书
│   ├── TEST_PLAN.md              #   测试计划
│   └── TEST_DESIGN.md            #   测试设计规格说明
├── package.json
├── CLAUDE.md                     # 开发指南
└── LICENSE                       # MIT License
```

---

## 数据流

```
[用户按下全局快捷键]
        │
        ▼
┌────────────────────────────────────────────────────────┐
│                    截图模块                              │
│  xcap 全屏截图 → 缓存到 CachedScreenStore                │
│  → 创建全屏蒙版窗口 → 用户拖拽框选 → 裁剪区域            │
│  → store_pin_image → 创建贴图窗口                       │
└────────────────────────────────────────────────────────┘
        │
        ▼
┌────────────────────────────────────────────────────────┐
│                   贴图窗口                               │
│  PinView 拉取图像 → 显示截图 + 控制栏                    │
│  用户点击「翻译」→ invoke translate_image                │
└────────────────────────────────────────────────────────┘
        │
        ▼
┌────────────────────────────────────────────────────────┐
│               OCR + 翻译流水线                           │
│  ① Tesseract 离线识别 → 文字 + 坐标 (OcrBlock[])        │
│  ② 查找历史缓存 ──→ 命中？──→ 直接返回缓存结果           │
│                    │                                    │
│                  未命中                                  │
│                    │                                    │
│  ③ 调用 AI API → 解析翻译 → 坐标对齐 → 返回翻译块      │
│  ④ 异步保存到 SQLite 历史记录                           │
└────────────────────────────────────────────────────────┘
        │
        ▼
[译文渲染到右侧面板，支持原文/译文切换、复制、拉伸]
```

---

## 设计理念

- **隐私优先：** OCR 在本地完成，不存在截图上传到第三方服务的风险；翻译仅与你自己的 API 端点通信，无遥测、无追踪
- **即截即用：** 截图固定在原位再翻译，不弹新窗口，不打断当前操作流
- **离线可靠：** 即使网络不可用，截图和贴图功能完全正常，OCR 完全离线
- **缓存提效：** 重复内容匹配历史缓存，秒出结果，节省 API 调用费用
- **轻量自持：** 基于 Tauri 构建，安装包小、内存占用低，Rust 后端保证高性能和低功耗

---

## 许可

[MIT License](LICENSE)

Copyright © 2026 XuMingKe
