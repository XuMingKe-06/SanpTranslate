import { invoke } from '@tauri-apps/api/core'

/** 应用配置，与后端 Rust AppConfig 结构体保持一致 */
export interface AppConfig {
  /** AI API 基础地址 */
  api_base_url: string
  /** AI 模型名称 */
  model: string
  /** 目标翻译语言 */
  target_language: string
  /** 快捷键配置 */
  shortcuts: ShortcutConfig
}

/** 快捷键配置 */
export interface ShortcutConfig {
  /** 截图快捷键 */
  capture: string
  /** 从剪贴板贴图快捷键 */
  pin_clipboard: string
}

/** 区域裁剪结果，包含图像数据和窗口位置信息 */
export interface CropResult {
  /** Base64 编码的 PNG 图像数据 */
  base64_data: string
  /** 贴图窗口 X 位置（逻辑像素） */
  x: number
  /** 贴图窗口 Y 位置（逻辑像素） */
  y: number
  /** 贴图窗口宽度（逻辑像素，含内边距） */
  width: number
  /** 贴图窗口高度（逻辑像素，含内边距和控制栏） */
  height: number
  /** 裁剪区域的物理像素宽度 */
  crop_width: number
  /** 裁剪区域的物理像素高度 */
  crop_height: number
}

/** OCR 文字块 */
export interface OcrBlock {
  /** 识别的文字 */
  text: string
  /** 左上角 X 坐标（百分比 0.0-1.0） */
  x: number
  /** 左上角 Y 坐标（百分比 0.0-1.0） */
  y: number
  /** 宽度（百分比 0.0-1.0） */
  width: number
  /** 高度（百分比 0.0-1.0） */
  height: number
}

/** 翻译结果块 */
export interface TranslatedBlock {
  /** 原始文本 */
  original: string
  /** 翻译后文本 */
  translated: string
  /** 左上角 X 坐标（百分比 0.0-1.0） */
  x: number
  /** 左上角 Y 坐标（百分比 0.0-1.0） */
  y: number
  /** 宽度（百分比 0.0-1.0） */
  width: number
  /** 高度（百分比 0.0-1.0） */
  height: number
}

/** 翻译结果，与后端 TranslateResult 对应 */
export interface TranslateResult {
  /** 翻译块列表 */
  blocks: TranslatedBlock[]
}

/** 历史记录列表条目 */
export interface HistoryListItem {
  /** 记录 ID */
  id: number
  /** 缩略图数据（Base64 编码的 JPEG） */
  thumbnail: string
  /** 翻译摘要 */
  summary: string
  /** 创建时间（ISO 8601 格式） */
  created_at: string
}

/** 历史记录详情条目 */
export interface HistoryEntry {
  /** 记录 ID */
  id: number
  /** 缩略图数据（Base64 编码的 JPEG） */
  thumbnail: string
  /** OCR 识别原文 */
  ocr_text: string | null
  /** 翻译后文本 */
  translated_text: string
  /** 创建时间（ISO 8601 格式） */
  created_at: string
}

export async function getConfig(): Promise<AppConfig> {
  return invoke<AppConfig>('get_config')
}

export async function saveConfig(config: AppConfig): Promise<void> {
  return invoke('save_config', { config })
}

export async function writeClipboardImage(imageData: string): Promise<void> {
  return invoke('write_clipboard_image', { imageData })
}

export async function readClipboardImage(): Promise<string | null> {
  return invoke<string | null>('read_clipboard_image')
}

export async function writeClipboardText(text: string): Promise<void> {
  return invoke('write_clipboard_text', { text })
}

export async function closePinWindow(windowId: string): Promise<void> {
  return invoke('close_pin_window', { windowId })
}

export async function getPinImage(windowId: string): Promise<string | null> {
  return invoke<string | null>('get_pin_image', { windowId })
}

// 从缓存的全屏截图中裁剪指定区域，返回裁剪结果（图像数据 + 位置信息）
export async function captureRegionFromCache(
  x: number,
  y: number,
  width: number,
  height: number
): Promise<CropResult> {
  return invoke<CropResult>('capture_region_from_cache', { x, y, width, height })
}

// 存储贴图图像数据到后端 PinImageStore，供 PinView 获取
export async function storePinImage(label: string, imageData: string): Promise<void> {
  return invoke('store_pin_image', { label, imageData })
}

/** 翻译图像，返回翻译结果 */
export async function translateImage(
  imageData: string,
  targetLanguage: string
): Promise<TranslateResult> {
  return invoke<TranslateResult>('translate_image', { imageData, targetLanguage })
}

/** 获取 API 密钥（从系统密钥环读取） */
export async function getApiKey(): Promise<string | null> {
  return invoke<string | null>('get_api_key')
}

/** 设置 API 密钥（保存到系统密钥环） */
export async function setApiKey(key: string): Promise<void> {
  return invoke('set_api_key', { key })
}

/** 测试 API 连接是否可用 */
export async function testApiConnection(
  apiBaseUrl: string,
  apiKey: string,
  model: string
): Promise<string> {
  return invoke<string>('test_api_connection', { apiBaseUrl, apiKey, model })
}

/** 获取历史记录列表 */
export async function getHistoryList(limit: number = 50): Promise<HistoryListItem[]> {
  return invoke<HistoryListItem[]>('get_history_list', { limit })
}

/** 获取历史记录详情 */
export async function getHistoryDetail(id: number): Promise<HistoryEntry> {
  return invoke<HistoryEntry>('get_history_detail', { id })
}

/** 删除指定历史记录 */
export async function deleteHistory(id: number): Promise<boolean> {
  return invoke<boolean>('delete_history', { id })
}

/** 清空所有历史记录 */
export async function clearHistory(): Promise<boolean> {
  return invoke<boolean>('clear_history')
}
