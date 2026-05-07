<template>
  <div
    class="pin-container"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @dblclick="onDoubleClick"
  >
    <!-- 内容行：左侧截图 + 右侧译文面板 -->
    <div class="content-row">
      <div class="image-area" ref="imageArea" :style="{ boxShadow: shadowStyle }">
        <img
          v-if="imageDataUrl"
          :src="imageDataUrl"
          class="pin-image"
          draggable="false"
          @load="onImageLoad"
        />
      </div>
      <!-- 译文面板 -->
      <div
        v-if="hasTranslation && !showOriginal"
        class="translation-panel"
      >
        <div
          v-for="(block, index) in filteredBlocks"
          :key="index"
          class="translation-item"
        >
          <div class="translation-text">{{ block.translated }}</div>
          <div v-if="index < filteredBlocks.length - 1" class="translation-separator"></div>
        </div>
      </div>
    </div>
    <ControlBar
      :translate-status="translateStatus"
      :show-original="showOriginal"
      :has-translation="hasTranslation"
      :error-message="errorMessage"
      :from-cache="fromCache"
      @translate="onTranslate"
      @copy-original="onCopyOriginal"
      @copy-translation="onCopyTranslation"
      @toggle-original="onToggleOriginal"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import {
  getPinImage,
  getConfig,
  translateImage,
  writeClipboardText,
  type TranslatedBlock,
} from '@/utils/tauri'
import { logger } from '@/utils/logger'
import ControlBar from '@/components/ControlBar.vue'

const TAG = 'PinView'

// 阴影内边距，需与后端 window/mod.rs 中的 PIN_PADDING 保持一致
const PIN_PADDING = 14
// 译文面板最大宽度
const MAX_PANEL_WIDTH = 340

type TranslateStatus = 'idle' | 'translating' | 'done' | 'error'

const imageDataUrl = ref<string>('')
const pinId = ref<string>('')
const translateStatus = ref<TranslateStatus>('idle')
const showOriginal = ref(false)
const hasTranslation = ref(false)

// 翻译相关状态
const translatedBlocks = ref<TranslatedBlock[]>([])
const errorMessage = ref<string>('')
const fromCache = ref(false)

// 过滤掉空翻译的块，避免在译文面板中显示空白项
const filteredBlocks = computed(() =>
  translatedBlocks.value.filter(b => b.translated.length > 0)
)

// 自适应阴影样式（根据图片边缘亮度选择暗色或亮色阴影）
const shadowStyle = ref('0 1px 5px 1px rgba(0,0,0,0.4)')

// 保存原始 base64 数据用于翻译
let rawBase64Data = ''

// 图片逻辑像素尺寸（用于窗口大小计算）
let logicalImageWidth = 0
let logicalImageHeight = 0
// 译文面板宽度（翻译完成后固定）
let storedPanelWidth = 0

const imageArea = ref<HTMLElement | null>(null)

let mouseDownX = 0
let mouseDownY = 0
let hasStartedDrag = false

/** HTML 转义，防止译文内容中出现 HTML 标签破坏布局 */
function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;')
}

/**
 * 离屏测量译文面板的自然宽度
 * 创建一个与面板样式相同的隐藏元素，测量其 scrollWidth 以确定内容宽度
 */
function measurePanelWidth(blocks: TranslatedBlock[]): number {
  const el = document.createElement('div')
  el.style.cssText = `
    position: fixed; left: -9999px; top: 0;
    font-size: 13px; line-height: 1.8;
    padding: 16px; max-width: ${MAX_PANEL_WIDTH}px;
    width: max-content;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    word-break: break-word; white-space: pre-wrap;
    color: #f0f0f0;
  `
  el.innerHTML = blocks.map((b, i) => {
    const text = escapeHtml(b.translated)
    const sep = i < blocks.length - 1
      ? '<div style="height:1px;margin:10px 0;background:rgba(255,255,255,0.08)"></div>'
      : ''
    return `<div style="margin-bottom:4px">${text}${sep}</div>`
  }).join('')

  document.body.appendChild(el)
  const w = Math.min(el.scrollWidth, MAX_PANEL_WIDTH)
  document.body.removeChild(el)
  return Math.max(w, 80) // 最小宽度 80px
}

/** 根据当前状态调整窗口大小 */
async function updateWindowSize(includePanel: boolean) {
  if (!logicalImageWidth || !logicalImageHeight) return

  const controlBarH = 36
  let width = logicalImageWidth + PIN_PADDING * 2
  const height = logicalImageHeight + controlBarH + PIN_PADDING * 2

  if (includePanel && storedPanelWidth > 0) {
    width += storedPanelWidth
  }

  try {
    await getCurrentWindow().setSize(new LogicalSize(width, height))
    logger.info(TAG, `窗口大小调整: ${width}x${height} (includePanel=${includePanel})`)
  } catch (err) {
    logger.error(TAG, `窗口大小调整失败: ${err}`, err)
  }
}

/**
 * 分析图片边缘像素亮度并设置自适应阴影
 * 通过 Canvas 提取图片四边（上下左右各 2 像素深度）的亮度值，
 * 若平均亮度低于阈值则使用亮色阴影（白色辉光），否则使用暗色阴影
 */
function analyzeEdgeBrightness(img: HTMLImageElement): void {
  try {
    const canvas = document.createElement('canvas')
    const w = img.naturalWidth
    const h = img.naturalHeight
    if (w === 0 || h === 0) return

    canvas.width = w
    canvas.height = h
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    ctx.drawImage(img, 0, 0)
    const imageData = ctx.getImageData(0, 0, w, h)
    const data = imageData.data

    let totalBrightness = 0
    let count = 0

    // 采样步长：图片越宽/高，步长越大，控制采样总数
    const stepX = Math.max(4, Math.floor(w / 150))
    const stepY = Math.max(4, Math.floor(h / 150))
    const depth = 2 // 边缘采样深度（像素行/列数）

    // 上下边缘
    for (let x = 0; x < w; x += stepX) {
      for (let d = 0; d < depth; d++) {
        // 上边缘
        if (d < h) {
          const idx = (d * w + x) * 4
          totalBrightness += (data[idx] + data[idx + 1] + data[idx + 2]) / 3
          count++
        }
        // 下边缘
        if (d < h) {
          const idx = ((h - 1 - d) * w + x) * 4
          totalBrightness += (data[idx] + data[idx + 1] + data[idx + 2]) / 3
          count++
        }
      }
    }

    // 左右边缘（跳过已采样的四角区域）
    for (let y = depth; y < h - depth; y += stepY) {
      for (let d = 0; d < depth; d++) {
        // 左边缘
        if (d < w) {
          const idx = (y * w + d) * 4
          totalBrightness += (data[idx] + data[idx + 1] + data[idx + 2]) / 3
          count++
        }
        // 右边缘
        if (d < w) {
          const idx = (y * w + (w - 1 - d)) * 4
          totalBrightness += (data[idx] + data[idx + 1] + data[idx + 2]) / 3
          count++
        }
      }
    }

    const avgBrightness = count > 0 ? totalBrightness / count / 255 : 0.5

    // 亮度阈值 0.45：边缘偏暗时使用亮色阴影，偏亮时使用暗色阴影
    if (avgBrightness < 0.45) {
      // 亮色阴影：白色描边 + 辉光，在暗色背景下清晰可见
      shadowStyle.value = '0 0 0 1px rgba(255,255,255,0.18), 0 0 10px 3px rgba(255,255,255,0.15)'
    } else {
      // 暗色阴影：默认的阴影效果，在亮色背景下清晰可见
      shadowStyle.value = '0 1px 5px 1px rgba(0,0,0,0.4)'
    }
  } catch (err) {
    // 分析失败时保持默认阴影
  }
}

async function onImageLoad(event: Event) {
  const img = event.target as HTMLImageElement
  if (!img || !img.naturalWidth || !img.naturalHeight) return

  const dpr = window.devicePixelRatio || 1
  logicalImageWidth = img.naturalWidth / dpr
  logicalImageHeight = img.naturalHeight / dpr

  logger.info(TAG, `图片加载完成: naturalSize=${img.naturalWidth}x${img.naturalHeight}, dpr=${dpr}, logicalSize=${logicalImageWidth}x${logicalImageHeight}`)

  // 分析边缘亮度以设置自适应阴影
  analyzeEdgeBrightness(img)

  await updateWindowSize(false)
}

onMounted(async () => {
  const currentWindow = getCurrentWindow()
  pinId.value = currentWindow.label
  logger.info(TAG, `PinView onMounted, windowLabel=${pinId.value}`)

  try {
    logger.info(TAG, `调用 getPinImage, windowId=${pinId.value}`)
    const base64Data = await getPinImage(pinId.value)
    if (base64Data) {
      logger.info(TAG, `获取到图片数据，长度=${base64Data.length}, startsWithData=${base64Data.startsWith('data:')}`)
      if (base64Data.startsWith('data:')) {
        // 去掉 data URI 前缀，保存纯 base64 数据
        rawBase64Data = base64Data.replace(/^data:image\/[^;]+;base64,/, '')
        imageDataUrl.value = base64Data
      } else {
        rawBase64Data = base64Data
        imageDataUrl.value = `data:image/png;base64,${base64Data}`
      }
      logger.info(TAG, `imageDataUrl 已设置，长度=${imageDataUrl.value.length}`)
    } else {
      logger.error(TAG, 'getPinImage 返回 null！图片数据未找到')
    }
  } catch (err) {
    logger.error(TAG, `getPinImage 调用失败: ${err}`, err)
  }
})

function onMouseDown(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (target.closest('.control-bar button')) return

  mouseDownX = e.clientX
  mouseDownY = e.clientY
  hasStartedDrag = false
}

async function onMouseMove(e: MouseEvent) {
  if (mouseDownX === 0 && mouseDownY === 0) return
  if (hasStartedDrag) return

  const dx = e.clientX - mouseDownX
  const dy = e.clientY - mouseDownY

  if (Math.abs(dx) > 3 || Math.abs(dy) > 3) {
    hasStartedDrag = true
    try {
      await getCurrentWindow().startDragging()
    } catch (err) {
      logger.error(TAG, `startDragging 失败: ${err}`, err)
    }
  }
}

function onMouseUp() {
  mouseDownX = 0
  mouseDownY = 0
  hasStartedDrag = false
}

async function onDoubleClick(event: MouseEvent) {
  const target = event.target as HTMLElement
  if (imageArea.value && imageArea.value.contains(target)) {
    try {
      logger.info(TAG, '双击关闭贴图窗口')
      await getCurrentWindow().destroy()
    } catch (err) {
      logger.error(TAG, `双击关闭失败: ${err}`, err)
    }
  }
}

// 调用后端翻译命令
async function onTranslate() {
  translateStatus.value = 'translating'
  errorMessage.value = ''

  try {
    // 获取配置以确定目标语言
    const config = await getConfig()
    logger.info(TAG, `开始翻译，目标语言=${config.target_language}`)

    // 调用翻译命令
    const result = await translateImage(rawBase64Data, config.target_language)

    if (!result.blocks || result.blocks.length === 0) {
      logger.info(TAG, '翻译结果为空，回到空闲状态')
      translateStatus.value = 'idle'
      return
    }

    // 保存翻译块列表
    translatedBlocks.value = result.blocks
    hasTranslation.value = true
    translateStatus.value = 'done'

    // 记录是否来自历史缓存
    fromCache.value = result.from_cache

    logger.info(TAG, `翻译完成，共 ${translatedBlocks.value.length} 个翻译块`)

    // 在下一帧测量面板宽度并调整窗口大小
    await nextTick()
    storedPanelWidth = measurePanelWidth(result.blocks)
    logger.info(TAG, `译文面板测量宽度: ${storedPanelWidth}px`)

    await updateWindowSize(true)
  } catch (err) {
    errorMessage.value = String(err)
    translateStatus.value = 'error'
    logger.error(TAG, `翻译失败: ${err}`, err)
  }
}

// 复制原文文本到剪贴板
async function onCopyOriginal() {
  if (filteredBlocks.value.length > 0) {
    const text = filteredBlocks.value.map(b => b.original).join('\n')
    try {
      await writeClipboardText(text)
      logger.info(TAG, '原文文本已复制到剪贴板')
    } catch (err) {
      logger.error(TAG, `复制原文失败: ${err}`, err)
    }
  }
}

// 复制译文文本到剪贴板
async function onCopyTranslation() {
  if (filteredBlocks.value.length > 0) {
    const text = filteredBlocks.value.map(b => b.translated).join('\n')
    try {
      await writeClipboardText(text)
      logger.info(TAG, '译文文本已复制到剪贴板')
    } catch (err) {
      logger.error(TAG, `复制译文失败: ${err}`, err)
    }
  }
}

// 切换原文/译文显示
async function onToggleOriginal() {
  showOriginal.value = !showOriginal.value
  // 切换后立即调整窗口大小
  await nextTick()
  await updateWindowSize(!showOriginal.value)
}
</script>

<style scoped>
.pin-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  padding: 14px;
  background: transparent;
  user-select: none;
}

.content-row {
  display: flex;
  flex-direction: row;
  flex: 1;
  min-height: 0;
}

.image-area {
  flex: 0 0 auto;
  overflow: hidden;
  position: relative;
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
}

.pin-image {
  display: block;
  width: 100%;
  height: 100%;
  object-fit: fill;
  pointer-events: none;
}

/* 译文面板 */
.translation-panel {
  background: rgba(30, 30, 30, 0.92);
  border-left: 1px solid rgba(255, 255, 255, 0.12);
  padding: 16px;
  max-width: 340px;
  overflow-y: auto;
  font-size: 13px;
  line-height: 1.8;
  color: #f0f0f0;
}

.translation-item {
  margin-bottom: 4px;
}

.translation-text {
  word-break: break-word;
  white-space: pre-wrap;
}

/* 翻译块之间的分隔线 */
.translation-separator {
  height: 1px;
  background: rgba(255, 255, 255, 0.08);
  margin: 10px 0;
}

/* 译文面板滚动条样式 */
.translation-panel::-webkit-scrollbar {
  width: 4px;
}

.translation-panel::-webkit-scrollbar-track {
  background: transparent;
}

.translation-panel::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
  border-radius: 2px;
}
</style>
