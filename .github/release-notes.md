# SnapTranslate ${{ github.ref_name }}

## v1.1.0 更新内容

- **新增：文本翻译窗口目标语言选择** — 在文本翻译窗口中可直接切换目标翻译语言，无需前往设置页面；该选择仅影响当前窗口，不回写全局配置
- **新增：贴图窗口 idle 状态下复制原文** — 未翻译时也可通过 OCR 识别并复制截图中的原文文字
- **改进：译文展示方式更新** — 译文通过右侧面板展示，支持原文/译文切换和面板高度拉伸（替代早期贴图覆盖方式）
- **改进：开发端口变更** — 开发服务器端口从默认 1420 更改为 51896，避免与其他 Tauri 项目冲突
- **修复：README 文档同步** — 更新所有 5 个语言版本的 README，修正译文展示方式描述，补充文本翻译目标语言功能说明

## 功能特性

- **框选截图翻译** — `Ctrl+Alt+L` 框选屏幕区域，自动贴图到原位
- **剪贴板贴图** — `Ctrl+Alt+P` 从剪贴板贴图翻译
- **文本翻译** — `Ctrl+Alt+M` 打开文本翻译窗口，支持自定义目标语言
- **本地 OCR** — 内置 Tesseract 离线引擎，无需联网
- **AI 翻译** — 支持任意 OpenAI 兼容 API
- **智能缓存** — 相同内容自动匹配缓存，跳过 API 调用
- **贴图窗口** — 截图原位固定，右侧译文面板，支持原文/译文切换、面板拉伸
- **翻译历史** — 本地 SQLite 存储，支持查看/复制/删除
- **双语界面** — 简体中文 / English，支持跟随系统
- **开机自启动** — 可选开机自动启动，随时待命

## 安装说明

### Windows
- 运行 `SnapTranslate_*_x64-setup.exe`（NSIS 安装包）
- 或双击 `SnapTranslate_*_x64_en-US.msi`（MSI 安装包）

### macOS
- 打开 `.dmg` 文件，将 SnapTranslate 拖入 Applications 文件夹

### Linux
```bash
# Debian/Ubuntu
sudo dpkg -i SnapTranslate_*_amd64.deb

# AppImage
chmod +x SnapTranslate_*_amd64.AppImage
./SnapTranslate_*_amd64.AppImage
```

## 使用前配置

首次使用需要配置 AI API：

1. 右键系统托盘图标 → **设置**
2. 填写 API 地址、密钥和模型名称
3. 选择目标翻译语言
4. 保存即可开始使用

> API 密钥通过操作系统凭据管理器安全存储，不会写入配置文件。

## 更新日志

请查看 [CHANGELOG](CHANGELOG.md) 获取详细更新内容。
