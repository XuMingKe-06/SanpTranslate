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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { captureRegionFromCache, storePinImage } from '@/utils/tauri'
import { logger } from '@/utils/logger'

const TAG = 'Overlay'

const canvasRef = ref<HTMLCanvasElement | null>(null)

const isSelecting = ref(false)
const startX = ref(0)
const startY = ref(0)
const endX = ref(0)
const endY = ref(0)
const fullscreenImage = ref<string | null>(null)
let fullscreenImgElement: HTMLImageElement | null = null

let keydownHandler: ((e: KeyboardEvent) => void) | null = null
let rafId: number | null = null

const selectionWidth = computed(() => Math.abs(endX.value - startX.value))
const selectionHeight = computed(() => Math.abs(endY.value - startY.value))

const sizeTipStyle = computed(() => {
  const rectX = Math.min(startX.value, endX.value)
  const rectY = Math.min(startY.value, endY.value)
  const h = selectionHeight.value
  return {
    left: `${rectX}px`,
    top: `${rectY + h + 4}px`,
  }
})

function drawCanvas() {
  const canvas = canvasRef.value
  if (!canvas || !fullscreenImgElement) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const dpr = window.devicePixelRatio || 1

  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.drawImage(fullscreenImgElement, 0, 0, canvas.width, canvas.height)
  ctx.fillStyle = 'rgba(0, 0, 0, 0.5)'
  ctx.fillRect(0, 0, canvas.width, canvas.height)

  if (isSelecting.value && selectionWidth.value > 0 && selectionHeight.value > 0) {
    const rx = Math.min(startX.value, endX.value) * dpr
    const ry = Math.min(startY.value, endY.value) * dpr
    const rw = selectionWidth.value * dpr
    const rh = selectionHeight.value * dpr

    ctx.save()
    ctx.beginPath()
    ctx.rect(rx, ry, rw, rh)
    ctx.clip()
    ctx.drawImage(fullscreenImgElement, 0, 0, canvas.width, canvas.height)
    ctx.restore()

    ctx.save()
    ctx.strokeStyle = '#ffffff'
    ctx.lineWidth = 1 * dpr
    ctx.setLineDash([4 * dpr, 4 * dpr])
    ctx.strokeRect(rx, ry, rw, rh)
    ctx.restore()
  }
}

function onMouseDown(e: MouseEvent) {
  if (!fullscreenImage.value) return
  e.preventDefault()
  isSelecting.value = true
  startX.value = e.clientX
  startY.value = e.clientY
  endX.value = e.clientX
  endY.value = e.clientY
}

function onMouseMove(e: MouseEvent) {
  if (!isSelecting.value) return
  endX.value = e.clientX
  endY.value = e.clientY
  if (rafId === null) {
    rafId = requestAnimationFrame(() => {
      drawCanvas()
      rafId = null
    })
  }
}

async function onMouseUp(e: MouseEvent) {
  if (!isSelecting.value) return
  isSelecting.value = false
  endX.value = e.clientX
  endY.value = e.clientY

  const dpr = window.devicePixelRatio || 1

  const cssX = Math.round(Math.min(startX.value, endX.value))
  const cssY = Math.round(Math.min(startY.value, endY.value))
  const cssW = Math.round(selectionWidth.value)
  const cssH = Math.round(selectionHeight.value)

  logger.info(TAG, `鼠标松开: cssX=${cssX}, cssY=${cssY}, cssW=${cssW}, cssH=${cssH}, dpr=${dpr}`)

  if (cssW < 5 || cssH < 5) {
    logger.info(TAG, '选区太小，忽略')
    drawCanvas()
    return
  }

  try {
    const physX = Math.round(cssX * dpr)
    const physY = Math.round(cssY * dpr)
    const physW = Math.round(cssW * dpr)
    const physH = Math.round(cssH * dpr)

    logger.info(TAG, `物理像素: physX=${physX}, physY=${physY}, physW=${physW}, physH=${physH}`)

    // 调用后端命令：从缓存裁剪区域 + 快速PNG编码 + 异步剪贴板写入
    logger.info(TAG, '调用 captureRegionFromCache...')
    const cropResult = await captureRegionFromCache(physX, physY, physW, physH)
    logger.info(TAG, `captureRegionFromCache 返回: x=${cropResult.x}, y=${cropResult.y}, w=${cropResult.width}, h=${cropResult.height}`)

    // 生成贴图窗口标签
    const label = `pin-${crypto.randomUUID()}`

    // 存储图像数据到后端 PinImageStore
    await storePinImage(label, cropResult.base64_data)
    logger.info(TAG, `图像数据已存储，label=${label}`)

    // 前端创建贴图窗口
    const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
    const pinWindow = new WebviewWindow(label, {
      url: '/pin',
      title: 'SnapTranslate - 贴图',
      decorations: false,
      alwaysOnTop: true,
      transparent: true,
      shadow: false,
      skipTaskbar: true,
      resizable: false,
      x: cropResult.x,
      y: cropResult.y,
      width: cropResult.width,
      height: cropResult.height,
    })

    pinWindow.once('tauri://created', () => {
      logger.info(TAG, `贴图窗口创建成功: label=${label}`)
    })

    pinWindow.once('tauri://error', (err) => {
      logger.error(TAG, `贴图窗口创建失败: ${err}`, err)
    })
  } catch (err) {
    logger.error(TAG, `框选处理失败: ${err}`, err)
  }

  logger.info(TAG, '尝试销毁 overlay 窗口...')
  try {
    await getCurrentWindow().destroy()
    logger.info(TAG, 'overlay 窗口销毁成功')
  } catch (err) {
    logger.error(TAG, `overlay 窗口销毁失败: ${err}`, err)
  }
}

function onKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    e.preventDefault()
    logger.info(TAG, '按下 Esc，销毁 overlay 窗口')
    getCurrentWindow().destroy().catch((err) => {
      logger.error(TAG, `Esc销毁窗口失败: ${err}`, err)
    })
  }
}

function onContextMenu(e: MouseEvent) {
  e.preventDefault()
  logger.info(TAG, '右键点击，销毁 overlay 窗口')
  getCurrentWindow().destroy().catch((err) => {
    logger.error(TAG, `右键销毁窗口失败: ${err}`, err)
  })
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
  fullscreenImage.value = dataUrl

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

  // 从后端拉取蒙版图像数据（后端先创建窗口再执行截图）
  try {
    const overlayData = await invoke<{ data: string; mime: string } | null>('get_overlay_image')
    if (overlayData) {
      logger.info(TAG, `拉取到蒙版图像数据，mime=${overlayData.mime}，数据长度=${overlayData.data.length}`)
      const dataUrl = `data:${overlayData.mime};base64,${overlayData.data}`
      loadFullscreenImage(dataUrl)
    } else {
      logger.error(TAG, '后端无蒙版图像数据')
    }
  } catch (err) {
    logger.error(TAG, `拉取蒙版图像数据失败: ${err}`, err)
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
  cursor: crosshair;
  overflow: hidden;
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
</style>
