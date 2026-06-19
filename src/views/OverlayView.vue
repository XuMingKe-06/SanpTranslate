<template>
  <div
    class="overlay-container"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @contextmenu.prevent="onContextMenu"
  >
    <canvas ref="canvasRef" />
    <div
      v-if="isSelecting && selectionWidth > 5 && selectionHeight > 5"
      class="size-tip"
      :style="sizeTipStyle"
    >
      {{ selectionWidth }} x {{ selectionHeight }}
    </div>
    <!-- 高可见性自定义光标：白色十字 + drop-shadow 暗色轮廓，在任何背景上都清晰可见 -->
    <div ref="cursorRef" class="custom-cursor">
      <svg width="20" height="20" viewBox="0 0 20 20">
        <line x1="10" y1="2" x2="10" y2="18" stroke="#fff" stroke-width="1.5" stroke-linecap="round"/>
        <line x1="2" y1="10" x2="18" y2="10" stroke="#fff" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { captureRegionFromCache, storePinImage } from '@/utils/tauri'
import { logger } from '@/utils/logger'

const TAG = 'Overlay'
const { t } = useI18n()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const cursorRef = ref<HTMLElement | null>(null)

// 用普通 JS 变量存储拖拽状态，完全绕过 Vue 响应式系统，消除拖拽卡顿
let _isSelecting = false
let _startX = 0
let _startY = 0
let _endX = 0
let _endY = 0
// 光标位置也用普通变量，避免响应式开销
let _cursorX = 0
let _cursorY = 0
let _cursorDirty = false
let fullscreenImgElement: HTMLImageElement | null = null

let keydownHandler: ((e: KeyboardEvent) => void) | null = null
let rafId: number | null = null

// 仅用于 size-tip UI 显示，低频更新
const isSelecting = ref(false)
const selectionWidth = ref(0)
const selectionHeight = ref(0)
const sizeTipLeft = ref(0)
const sizeTipTop = ref(0)

const sizeTipStyle = computed(() => ({
  left: `${sizeTipLeft.value}px`,
  top: `${sizeTipTop.value}px`,
}))

function drawCanvas() {
  const canvas = canvasRef.value
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const dpr = window.devicePixelRatio || 1
  const cw = canvas.width
  const ch = canvas.height

  // 有截图时绘制背景图，否则只显示半透明遮罩（只绘制一次，不 clearRect 避免闪烁）
  if (fullscreenImgElement) {
    ctx.drawImage(fullscreenImgElement, 0, 0, cw, ch)
  } else {
    ctx.clearRect(0, 0, cw, ch)
  }

  // 直接读取普通 JS 变量，无响应式开销
  const rx = Math.min(_startX, _endX) * dpr
  const ry = Math.min(_startY, _endY) * dpr
  const rw = Math.abs(_endX - _startX) * dpr
  const rh = Math.abs(_endY - _startY) * dpr

  ctx.fillStyle = 'rgba(0, 0, 0, 0.5)'

  if (_isSelecting && rw > 0 && rh > 0) {
    // 4 个矩形遮住选区外部，比 clip+再绘大图 快 5-10x
    ctx.fillRect(0, 0, cw, ry)               // 顶部
    ctx.fillRect(0, ry + rh, cw, ch - ry - rh) // 底部
    ctx.fillRect(0, ry, rx, rh)              // 左侧
    ctx.fillRect(rx + rw, ry, cw - rx - rw, rh) // 右侧

    // 选区虚线框
    ctx.save()
    ctx.strokeStyle = 'rgba(255,255,255,0.9)'
    ctx.lineWidth = 1.5 * dpr
    ctx.setLineDash([5 * dpr, 3 * dpr])
    ctx.strokeRect(rx, ry, rw, rh)
    ctx.restore()
  } else {
    ctx.fillRect(0, 0, cw, ch)
  }
}

/** 更新自定义光标 DOM 位置（在 rAF 中调用，避免 Vue ref 响应式开销） */
function updateCursorPosition() {
  if (!_cursorDirty) return
  _cursorDirty = false
  const el = cursorRef.value
  if (el) {
    el.style.left = `${_cursorX}px`
    el.style.top = `${_cursorY}px`
  }
}

function onMouseDown(e: MouseEvent) {
  e.preventDefault()
  _isSelecting = true
  isSelecting.value = true
  _startX = _endX = e.clientX
  _startY = _endY = e.clientY
  selectionWidth.value = 0
  selectionHeight.value = 0
}

function onMouseMove(e: MouseEvent) {
  // 更新光标位置（普通变量，无响应式开销）
  _cursorX = e.clientX
  _cursorY = e.clientY
  _cursorDirty = true

  if (!_isSelecting) {
    // 未在框选时仍需更新光标位置
    if (rafId === null) {
      rafId = requestAnimationFrame(() => {
        updateCursorPosition()
        rafId = null
      })
    }
    return
  }
  _endX = e.clientX
  _endY = e.clientY
  // 低频更新 Vue ref，仅用于 size-tip 显示（不影响 canvas 渲染速度）
  const w = Math.abs(_endX - _startX)
  const h = Math.abs(_endY - _startY)
  if (w > 5 && h > 5) {
    selectionWidth.value = Math.round(w)
    selectionHeight.value = Math.round(h)
    sizeTipLeft.value = Math.min(_startX, _endX)
    sizeTipTop.value = Math.min(_startY, _endY) + h + 4
  }
  // canvas 渲染走 rAF，完全不受 Vue 响应式影响
  if (rafId === null) {
    rafId = requestAnimationFrame(() => {
      drawCanvas()
      updateCursorPosition()
      rafId = null
    })
  }
}

async function onMouseUp(e: MouseEvent) {
  if (!_isSelecting) return
  _isSelecting = false
  isSelecting.value = false
  _endX = e.clientX
  _endY = e.clientY

  const dpr = window.devicePixelRatio || 1

  const cssX = Math.round(Math.min(_startX, _endX))
  const cssY = Math.round(Math.min(_startY, _endY))
  const cssW = Math.round(Math.abs(_endX - _startX))
  const cssH = Math.round(Math.abs(_endY - _startY))

  logger.info(TAG, `鼠标松开: cssX=${cssX}, cssY=${cssY}, cssW=${cssW}, cssH=${cssH}, dpr=${dpr}`)

  if (cssW < 5 || cssH < 5) {
    logger.info(TAG, '选区太小，忽略')
    drawCanvas()
    return
  }

  // 清除选区矩形，显示纯遮罩
  drawCanvas()

  const physX = Math.round(cssX * dpr)
  const physY = Math.round(cssY * dpr)
  const physW = Math.round(cssW * dpr)
  const physH = Math.round(cssH * dpr)

  logger.info(TAG, `物理像素: physX=${physX}, physY=${physY}, physW=${physW}, physH=${physH}`)

  // 直接从 CSS 坐标计算贴图窗口位置（无需等待后端编码结果）
  const PIN_PADDING = 14
  const CONTROL_BAR_H = 36
  const windowX = cssX - PIN_PADDING
  const windowY = cssY - PIN_PADDING
  const windowWidth = cssW + PIN_PADDING * 2
  const windowHeight = cssH + CONTROL_BAR_H + PIN_PADDING * 2

  // 创建贴图窗口（定位到正确位置，visible），与后端编码并行
  const label = `pin-${crypto.randomUUID()}`
  const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')

  const pinWindow = new WebviewWindow(label, {
    url: '/pin',
    title: t('pin.title'),
    decorations: false,
    alwaysOnTop: true,
    transparent: true,
    shadow: false,
    skipTaskbar: true,
    resizable: false,
    x: windowX,
    y: windowY,
    width: windowWidth,
    height: windowHeight,
  })

  pinWindow.once('tauri://error', (err) => {
    logger.error(TAG, `贴图窗口创建失败: ${err}`, err)
  })

  // 裁剪编码 + 存储图像，完成后再销毁蒙版
  // 必须等 IPC 完成再 destroy，否则窗口 JS 上下文被销毁导致后续 await 中断
  try {
    const cropResult = await captureRegionFromCache(physX, physY, physW, physH)
    logger.info(TAG, `captureRegionFromCache 返回: x=${cropResult.x}, y=${cropResult.y}, w=${cropResult.width}, h=${cropResult.height}`)

    await storePinImage(label, cropResult.base64_data)
    logger.info(TAG, `图像数据已存储，label=${label}`)
  } catch (err) {
    logger.error(TAG, `框选处理失败: ${err}`, err)
  }

  // IPC 完成后关闭蒙版（DWM 动画已在窗口创建时禁用）
  await closeOverlay()
}

async function closeOverlay() {
  const win = getCurrentWindow()
  try {
    await win.destroy()
  } catch (err) {
    logger.error(TAG, `关闭蒙版失败: ${err}`, err)
  }
}

function onKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    e.preventDefault()
    logger.info(TAG, '按下 Esc，关闭 overlay 窗口')
    closeOverlay()
  }
}

function onContextMenu(e: MouseEvent) {
  e.preventDefault()
  logger.info(TAG, '右键点击，关闭 overlay 窗口')
  closeOverlay()
}

function initCanvasSize() {
  const canvas = canvasRef.value
  if (!canvas) return
  const dpr = window.devicePixelRatio || 1
  canvas.width = window.innerWidth * dpr
  canvas.height = window.innerHeight * dpr
  logger.info(TAG, `Canvas初始化: innerSize=${window.innerWidth}x${window.innerHeight}, dpr=${dpr}, canvasSize=${canvas.width}x${canvas.height}`)
}

function loadFullscreenImage(dataUrl: string) {
  logger.info(TAG, `收到全屏截图数据，dataUrl长度=${dataUrl.length}`)
  const img = new Image()
  img.onload = () => {
    fullscreenImgElement = img
    logger.info(TAG, `全屏截图Image加载完成: naturalSize=${img.naturalWidth}x${img.naturalHeight}`)
    drawCanvas()
    logger.info(TAG, 'Canvas绘制完成')
  }
  img.onerror = (err) => {
    logger.error(TAG, `全屏截图Image加载失败`, err)
  }
  img.src = dataUrl
}

onMounted(async () => {
  logger.info(TAG, 'OverlayView onMounted')
  initCanvasSize()
  // 立即绘制半透明遮罩，不等截图加载完成，让用户感知蒙版已响应
  drawCanvas()

  // 轮询拉取蒙版图像数据（后台线程异步编码截图）
  const POLL_INTERVAL_MS = 100
  const MAX_POLL_ATTEMPTS = 50  // 50 * 100ms = 5s 超时
  let imageLoaded = false

  for (let i = 0; i < MAX_POLL_ATTEMPTS; i++) {
    try {
      const overlayData = await invoke<{ data: string; mime: string } | null>('get_overlay_image')
      if (overlayData) {
        logger.info(TAG, `拉取到蒙版图像数据（第${i + 1}次轮询），mime=${overlayData.mime}，数据长度=${overlayData.data.length}`)
        const dataUrl = `data:${overlayData.mime};base64,${overlayData.data}`
        loadFullscreenImage(dataUrl)
        imageLoaded = true
        break
      }
    } catch (err) {
      logger.error(TAG, `拉取蒙版图像数据失败: ${err}`, err)
      break  // 命令出错不再重试
    }

    if (i < MAX_POLL_ATTEMPTS - 1) {
      await new Promise(resolve => setTimeout(resolve, POLL_INTERVAL_MS))
    }
  }

  // 最终额外尝试一次，覆盖极微小的竞态窗口
  if (!imageLoaded) {
    try {
      const overlayData = await invoke<{ data: string; mime: string } | null>('get_overlay_image')
      if (overlayData) {
        logger.info(TAG, `最终尝试拉取到蒙版图像数据`)
        const dataUrl = `data:${overlayData.mime};base64,${overlayData.data}`
        loadFullscreenImage(dataUrl)
        imageLoaded = true
      }
    } catch { /* 忽略最终尝试的异常 */ }
  }

  if (!imageLoaded) {
    logger.error(TAG, `轮询超时（${MAX_POLL_ATTEMPTS * POLL_INTERVAL_MS}ms），未获取到蒙版图像数据`)
  }

  keydownHandler = onKeyDown
  window.addEventListener('keydown', keydownHandler)
  logger.info(TAG, 'OverlayView 初始化完成')
})

onUnmounted(() => {
  logger.info(TAG, 'OverlayView onUnmounted')
  if (keydownHandler) {
    window.removeEventListener('keydown', keydownHandler)
    keydownHandler = null
  }
  if (rafId !== null) {
    cancelAnimationFrame(rafId)
    rafId = null
  }
  fullscreenImgElement = null
})
</script>

<style scoped>
.overlay-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  cursor: none;
}

.overlay-container canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.size-tip {
  position: fixed;
  font-size: 12px;
  color: #ffffff;
  background: rgba(0, 0, 0, 0.7);
  padding: 2px 6px;
  border-radius: 0;
  pointer-events: none;
  white-space: nowrap;
  z-index: 10;
}

.custom-cursor {
  position: fixed;
  pointer-events: none;
  z-index: 9999;
  transform: translate(-50%, -50%);
  filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0.8));
}

.custom-cursor svg {
  display: block;
}
</style>
