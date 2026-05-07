# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

- 在回答过程中请使用中文
- 在代码中添加中文注释

***

## Reasoning Effort: 
Absolute maximum with no shortcuts permitted.
You MUST be very thorough in your thinking and comprehensively decompose the problem to resolve the root cause, rigorously stress-testing your logic against all potential paths, edge cases, and adversarial scenarios.
Explicitly write out your entire deliberation process, documenting every intermediate step, considered alternative, and rejected hypothesis to ensure absolutely no assumption is left unchecked.

## 项目概述

SnapTranslate 是一款基于 Tauri 2.x 的桌面截屏翻译工具。它能截取屏幕区域、执行 OCR（Tesseract）识别文字并调用 AI 翻译，将译文覆盖在原截图上方，以贴图形式固定在桌面上。

**当前状态：** S4 阶段进行中 — 截图、剪贴板、快捷键、贴图窗口、框选蒙版、托盘菜单、OCR、翻译、设置页面均已实现。历史记录模块为桩代码，国际化文件已创建但内容为空。

## 开发命令

```bash
# 开发模式（Vite HMR + Tauri）
npm run tauri dev

# 构建生产版本
npm run tauri build

# 仅前端（Vite 开发服务器）
npm run dev

# 前端类型检查
npx vue-tsc --noEmit

# Vite 预览
npm run preview
```

## 架构

### 运行时流程

1. 用户按下全局快捷键（默认 `Ctrl+Alt+L`）→ 截图模块截取屏幕
2. 截图以透明窗口形式固定在原位置（贴图窗口）
3. 用户点击"AI 翻译"→ 本地 Tesseract OCR 提取文本及坐标 → 调用 AI API 翻译文本
4. 译文以标签形式覆盖在原文本位置上
5. 翻译记录保存到本地 SQLite 数据库（待实现）

### Rust 后端（`src-tauri/src/`）

| 模块          | 文件                  | 状态                                                                                                                                                                                                  |
| ----------- | ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capture`   | `capture/mod.rs`    | **已完成** — `CaptureService` 封装 xcap，支持全屏截图和区域截图，返回 Base64 PNG/JPEG；包含 `MonitorInfo`、`CaptureRegion`、`CapturedImage` 数据结构；支持缓存截图数据供区域裁剪使用                                                                                  |
| `ocr`       | `ocr/mod.rs`        | **已完成** — 使用 Tesseract CLI 进行本地文字识别，支持从资源目录或系统 PATH 查找 Tesseract；`extract_text_with_positions` 提取文字及坐标，TSV 解析将词级结果合并为行级块；包含 `OcrBlock` 数据结构（百分比坐标） |
| `translate` | `translate/mod.rs`  | **已完成** — OCR 模式翻译：本地 Tesseract 提取文字及坐标 → 调用文本模型 API 翻译 → 合并坐标返回；包含 `translate_image` 入口函数、`call_text_api`（OpenAI 兼容格式）、`TranslatedBlock`/`TranslateResult` 数据结构 |
| `clipboard` | `clipboard/mod.rs`  | **已完成** — `read_clipboard_image`/`write_clipboard_image`/`write_clipboard_text`，支持 Base64 和原始 RGBA 数据读写图片                                                                                |
| `hotkey`    | `hotkey/mod.rs`     | **已完成** — `register_hotkeys` 注册全局快捷键（从配置动态解析），支持 Ctrl/Shift/Alt/Super 修饰键 + A-Z/0-9/F1-F12，回调中串联截图或剪贴板操作                                                                                            |
| `history`   | `history/mod.rs`    | 桩代码 — 仅有注释 `// 历史模块`，将使用 `rusqlite` 进行 SQLite 存储                                                                                                                                                                   |
| `config`    | `config/manager.rs` | **已完成** — 基于 TOML 的配置（API URL、模型名称、目标语言、快捷键），从 `app_config_dir/config.toml` 加载，通过临时文件+重命名实现原子写入；支持通过 keyring 管理 API 密钥                                                                                                       |
| `config`    | `config/mod.rs`     | 重新导出 `AppConfig`、`ConfigManager`、`ShortcutConfig`                                                                                                                                   |
| `window`    | `window/mod.rs`     | **已完成** — `create_settings_window`/`create_history_window`（单例模式）、`create_overlay_window`（全屏蒙版，将图像数据存入缓存供前端拉取）、`create_pin_window`（UUID 标签，窗口尺寸预留控制栏高度）、`close_pin_window`、`get_pin_image`、`PinImageStore`/`CachedScreenStore`/`CachedScreen`/`OverlayImageData`/`CropResult` 数据结构 |
| `tray`      | `tray/mod.rs`       | **已完成** — 系统托盘菜单：框选截图翻译、从剪贴板贴图、翻译最近贴图、历史记录、设置、退出；所有菜单项已接入实际逻辑                                                                                                                  |
| `commands`  | `commands.rs`       | **已完成** — 十四个 Tauri 命令：`get_config`、`save_config`、`write_clipboard_image`、`read_clipboard_image`、`write_clipboard_text`、`close_pin_window`、`get_pin_image`、`capture_region_from_cache`、`get_overlay_image`、`store_pin_image`、`translate_image`、`get_api_key`、`set_api_key`、`test_api_connection` |
| `error`     | `error.rs`          | **已完成** — 统一的 `AppError` 枚举，`Display` 输出中文错误信息，包含 `io`、`toml`、`reqwest`、`rusqlite`、`tauri` 的 `From` 实现                                                                                              |
| `lib.rs`    | —                   | 应用入口点：注册 opener、clipboard\_manager、log 插件；管理 `PinImageStore` 和 `CachedScreenStore` 状态；注册所有命令；在 `setup()` 中初始化截图服务、创建系统托盘并注册全局快捷键                                                                              |
| `main.rs`   | —                   | `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`，调用 `snap_translate_lib::run()`                                                                                                 |

**关键后端模式：** 所有模块在其目录下均为扁平化的 `mod.rs` 文件。目前只有 config 采用了拆分模块结构（config/mod.rs + config/manager.rs）。

**主要依赖库：** tauri 2、tauri-plugin-opener、tauri-plugin-clipboard-manager、tauri-plugin-global-shortcut、tauri-plugin-log、serde、serde_json、xcap、reqwest、toml、keyring、rusqlite、image、base64、uuid、log

### 前端（`src/`）

| 层级    | 文件                          | 用途                                                                            |
| ----- | --------------------------- | ----------------------------------------------------------------------------- |
| 入口    | `main.ts`                   | 创建 Vue 应用，注册 router/Pinia/i18n                                                |
| 路由    | `router/index.ts`           | 四个路由：`/overlay`（截图蒙版）、`/pin`（贴图）、`/settings`、`/history`                       |
| 组件    | `components/ControlBar.vue` | **已完成** — 贴图控制栏组件：根据翻译状态（idle/translating/done/error）显示 AI 翻译按钮、翻译中状态、复制全部/切换原文译文按钮组；支持错误重试 |
| Pinia | `stores/configStore.ts`     | **已完成** — 配置状态管理（通过 `invoke` 与 Rust 后端进行加载/保存），包含 `loadApiKey`/`setApiKey` 方法管理 API 密钥（从 keyring 读取/写入） |
| Pinia | `stores/pinStore.ts`        | 贴图状态管理（`TranslatedBlock`、`PinState` 及贴图实例的 Map）                               |
| Pinia | `stores/historyStore.ts`    | 历史记录状态（桩代码，S5 阶段实现）                                                                   |
| 国际化   | `i18n/index.ts`             | `vue-i18n` 配置，自动检测 zh-CN 或 en-US                                              |
| 国际化   | `i18n/locales/zh-CN.ts`     | 中文语言文件（已创建，当前为空对象）                                                                 |
| 国际化   | `i18n/locales/en-US.ts`     | 英文语言文件（已创建，当前为空对象）                                                                 |
| 工具函数  | `utils/tauri.ts`            | **已完成** — Tauri 命令的 TypeScript 绑定（所有十四个命令均已覆盖），包含 `AppConfig`、`CropResult`、`OcrBlock`、`TranslatedBlock`、`TranslateResult` 等接口定义 |
| 工具函数  | `utils/logger.ts`           | 日志工具，封装 `@tauri-apps/plugin-log`，提供带时间戳和标签的 debug/info/warn/error 结构化日志输出 |
| 样式    | `styles/variables.css`      | CSS 自定义属性（深色透明主题）                                                             |
| 样式    | `styles/global.css`         | 全局重置及基础样式                                                                     |
| 视图    | `views/OverlayView.vue`     | **已完成** — Canvas 全屏截图蒙版，支持鼠标框选（白虚线框+暗色蒙版）、尺寸提示、Esc 关闭；选后调用后端裁剪+写入剪贴板+创建贴图窗口   |
| 视图    | `views/PinView.vue`         | **已完成** — 贴图窗口：显示截图、控制栏组件（翻译/复制/切换）、原生窗口拖拽（排除按钮区域）、双击图片区域关闭；集成 OCR 翻译功能、翻译覆盖层显示、原文/译文切换、复制翻译文本；监听托盘"翻译最近贴图"事件 |
| 视图    | `views/SettingsView.vue`    | **已完成** — 设置页面：使用 Naive UI 组件库，包含 API 配置（地址/密钥/模型）、翻译配置（目标语言）、快捷键配置；支持保存配置和测试 API 连接；API 密钥通过 keyring 管理 |
| 视图    | `views/HistoryView.vue`     | 历史记录页面（桩代码）                                                                   |

**关键前端模式：** 使用 `@/` 路径别名（在 vite.config.ts 和 tsconfig.json 中均已配置）。通过 `@tauri-apps/api/core` 的 `invoke()` 进行 Tauri IPC 通信。窗口间数据传递采用"后端缓存 + 前端主动拉取"模式（非 Event 推送）。所有视图均为懒加载。

**主要依赖库：** vue 3.5、vue-router 5、pinia 3、vue-i18n 11、naive-ui 2.44、@tauri-apps/api 2、@tauri-apps/plugin-log

### 前端通信模式

- **蒙版窗口流程：** `lib.rs` setup → 快捷键/托盘回调 → `capture::capture_fullscreen_with_cache()` → 缓存全屏截图到 `CachedScreenStore.screen` → `window::create_overlay_window()` → 后端将 JPEG 图像数据存入 `CachedScreenStore.overlay_image` → OverlayView 调用 `get_overlay_image` 命令拉取数据 → 绘制截图 → 用户框选 → `capture_region_from_cache` 命令 → 返回 `CropResult`（含图像和位置信息）→ `store_pin_image` 存储图像 → 创建贴图窗口
- **贴图窗口流程：** `create_pin_window()` 创建 WebviewWindow → PinView 调用 `get_pin_image` 命令从 `PinImageStore` 拉取图像数据 → 显示图片 + ControlBar
- **翻译流程：** 用户点击"AI 翻译"按钮 → PinView 调用 `get_config` 获取目标语言 → 调用 `translate_image` 命令 → 后端执行 Tesseract OCR 提取文字及坐标 → 调用文本模型 API 翻译 → 返回 `TranslateResult`（包含 `TranslatedBlock[]`）→ 前端渲染翻译覆盖层

### AppConfig 结构（Rust）

```rust
pub struct AppConfig {
    pub api_base_url: String,      // AI API 基础地址
    pub model: String,             // AI 模型名称，例如 "gpt-4o"
    pub target_language: String,   // 目标翻译语言，默认 "zh-CN"
    pub shortcuts: ShortcutConfig, // 快捷键配置
}

pub struct ShortcutConfig {
    pub capture: String,        // 截图翻译快捷键，默认 "Ctrl+Alt+L"
    pub pin_clipboard: String,  // 从剪贴板贴图快捷键，默认 "Ctrl+Alt+P"
}
```

### 关键设计决策

- **配置存储：** TOML 文件存储在磁盘上，API 密钥通过操作系统凭据管理器（`keyring` crate）保存 — 不写入配置文件
- **翻译流程：** 本地 Tesseract OCR 提取文字及坐标 → 拼接文本调用 AI API 翻译 → 按行匹配坐标返回翻译块
- **Tesseract 资源：** 项目捆绑 Tesseract 可执行文件和语言数据（`src-tauri/resources/tesseract/`），包含中文简体（`chi_sim.traineddata`）和英文（`eng.traineddata`）训练数据，以及 Windows DLL 依赖；OCR 模块优先从资源目录查找，开发模式下回退到系统 PATH
- **贴图窗口：** 每张贴图截图都是一个独立的透明 Tauri Webview 窗口，定位在原始截取坐标处，窗口尺寸 = 图片尺寸 + 4px 内边距 + 36px 控制栏高度
- **图像缓存：** 全屏截图缓存于 `CachedScreenStore`（含 `screen` 和 `overlay_image`），贴图图像缓存于 `PinImageStore`，前端通过命令主动拉取而非 Event 推送
- **错误处理：** 统一的 `AppError` 枚举，`Display` 输出中文错误信息，包含所有主要错误类型的 `From` 实现
- **窗口管理：** 设置和历史记录窗口为单例（如果已打开则复用现有窗口）；蒙版窗口为单例（打开前先关闭已有实例）；贴图窗口每次创建新实例（UUID 标签）
- **UI 框架：** Naive UI 已在设置页面中使用，采用深色主题配合 `createDiscreteApi` 创建独立的 message 实例
- **日志系统：** 使用 `tauri-plugin-log`，输出到 Stdout、Webview 控制台和日志文件目录
