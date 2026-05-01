# SnapTranslate

极简、高效、隐私安全的桌面截屏贴图翻译工具。

## 功能特性

- **框选截图**：全局快捷键 `Ctrl+Shift+X`，拖拽框选屏幕任意区域
- **原位贴图**：截图自动以置顶窗口形式贴在截取原位
- **剪贴板贴图**：`Ctrl+Shift+V` 从剪贴板取图贴到桌面
- **本地 OCR**：内置 Tesseract 离线文字识别，提取文本及精确坐标
- **AI 翻译**：调用自备大模型 API 翻译，支持 OCR 模式与多模态模式
- **译文覆盖**：OCR 模式下译文标签精确覆盖原文位置
- **翻译历史**：自动记录翻译历史，支持查看、复制、删除
- **隐私安全**：截图完全本地处理，仅与用户配置的 API 通信，无遥测

## 技术栈

| 层级         | 技术                            |
|-------------|--------------------------------|
| 桌面框架     | Tauri 2.x                      |
| 前端         | Vue 3 + TypeScript + Vite      |
| UI 组件      | Naive UI                       |
| 后端         | Rust                           |
| 屏幕截图     | xcap                           |
| OCR          | Tesseract (via leptess)        |
| 数据库       | SQLite (via rusqlite)          |
| 安全存储     | keyring                        |
| HTTP 客户端  | reqwest                        |
| 状态管理     | Pinia                          |
| 国际化       | vue-i18n                       |

## 环境要求

### 开发环境

- Node.js >= 18
- Rust >= 1.85
- Tauri CLI 2.x

### 运行环境

- Windows 10 (1803+) / macOS 12+ / Linux (X11/Wayland)
- WebView2 (Windows) / WebKit (macOS/Linux)
- Tesseract OCR 运行时库

## 快速开始

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI
npm install -g @tauri-apps/cli
```

### 开发模式

```bash
npm run tauri dev
```

### 构建发布

```bash
npm run tauri build
```

## 项目结构

```
SnapTranslate/
  |-- src/                    # 前端源码 (Vue 3 + TypeScript)
  |     |-- components/       # Vue 组件
  |     |-- views/            # 页面视图
  |     |-- stores/           # Pinia 状态管理
  |     |-- i18n/             # 国际化语言包
  |     |-- styles/           # 全局样式
  |     |-- utils/            # 工具函数
  |-- src-tauri/              # Rust 后端源码
  |     |-- src/
  |     |     |-- capture/    # 截图模块
  |     |     |-- ocr/        # OCR 模块
  |     |     |-- translate/  # 翻译模块
  |     |     |-- config/     # 配置模块
  |     |     |-- history/    # 历史模块
  |     |     |-- clipboard/  # 剪贴板模块
  |     |     |-- hotkey/     # 快捷键模块
  |     |     |-- tray/       # 托盘模块
  |     |     |-- window/     # 窗口管理模块
  |-- docs/                   # 项目文档
  |-- tests/                  # 测试代码
```

## 使用说明

1. 启动应用后，系统托盘出现图标
2. 右键托盘图标 -> "设置"，配置大模型 API 地址和密钥
3. 按 `Ctrl+Shift+X` 框选截图，截图自动贴在原位
4. 点击贴图下方"AI翻译"按钮进行翻译
5. 按 `Ctrl+Shift+V` 可将剪贴板中的图片贴到桌面

## 配置说明

配置文件位于：
- Windows: `%APPDATA%/SnapTranslate/config/config.toml`
- macOS: `~/Library/Application Support/SnapTranslate/config/config.toml`
- Linux: `~/.config/SnapTranslate/config/config.toml`

API 密钥通过操作系统凭据管理器安全存储，不保存在配置文件中。

## 文档

- [软件需求规格说明书](docs/SRS.md)
- [系统/架构设计文档](docs/ARCHITECTURE.md)
- [概要设计说明书](docs/HLD.md)
- [详细设计说明书](docs/DLD.md)
- [测试计划](docs/TEST_PLAN.md)
- [测试设计规格说明](docs/TEST_DESIGN.md)

## 许可证

MIT License
