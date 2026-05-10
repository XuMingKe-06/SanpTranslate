<template>
  <div @contextmenu.prevent>
    <router-view />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getConfig } from '@/utils/tauri'
import { logger } from '@/utils/logger'

const TAG = 'App'
const { locale } = useI18n()

let unlisten: UnlistenFn | null = null

/** 根据配置设置界面语言 */
function applyLanguage(language: string) {
  if (language === 'auto') {
    // 跟随系统：使用 navigator.language 自动检测（已在 i18n 初始化时设置）
    const sysLang = navigator.language.startsWith('zh') ? 'zh-CN' : 'en-US'
    locale.value = sysLang
  } else {
    locale.value = language
  }
}

onMounted(async () => {
  // 从后端加载配置，初始化界面语言
  try {
    const config = await getConfig()
    applyLanguage(config.language)
    logger.info(TAG, `界面语言初始化: config.language=${config.language}, locale=${locale.value}`)
  } catch (err) {
    logger.error(TAG, `加载配置初始化语言失败: ${err}`)
  }

  // 监听后端广播的语言变更事件（其他窗口保存配置时触发）
  try {
    unlisten = await listen<string>('language-changed', (event) => {
      const newLang = event.payload
      locale.value = newLang
      logger.info(TAG, `收到语言变更事件，切换到: ${newLang}`)
    })
  } catch (err) {
    logger.error(TAG, `注册语言变更监听失败: ${err}`)
  }
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
    unlisten = null
  }
})
</script>
