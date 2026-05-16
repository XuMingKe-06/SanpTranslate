<template>
  <div class="control-bar">
    <!-- idle 或 error 状态：显示 AI 翻译主按钮 + 复制原文按钮 -->
    <template v-if="translateStatus === 'idle' || translateStatus === 'error'">
      <button
        class="btn"
        @click="$emit('translate')"
      >
        {{ translateStatus === 'error' ? t('controlBar.retranslate') : t('controlBar.translate') }}
      </button>

      <!-- 复制原文按钮（idle 状态下通过 OCR 识别获取文字） -->
      <button
        class="btn"
        :disabled="ocrLoading"
        @click="$emit('ocrCopyOriginal')"
      >
        {{ ocrLoading ? t('controlBar.recognizing') : t('controlBar.copyOriginal') }}
      </button>
    </template>

    <!-- error 状态：显示错误提示信息 -->
    <span v-if="translateStatus === 'error' && errorMessage" class="error-msg">
      {{ errorMessage }}
    </span>

    <!-- translating 状态：显示禁用的翻译中按钮 -->
    <button
      v-else-if="translateStatus === 'translating'"
      class="btn"
      disabled
    >
      {{ t('controlBar.translating') }}
    </button>

    <!-- done 状态：显示操作按钮组 -->
    <template v-else-if="translateStatus === 'done'">
      <!-- 复制原文 -->
      <button class="btn" @click="$emit('copyOriginal')">{{ t('controlBar.copyOriginal') }}</button>

      <!-- 复制译文 -->
      <button class="btn" @click="$emit('copyTranslation')">{{ t('controlBar.copyTranslation') }}</button>

      <!-- 重新翻译（强制调用API，跳过缓存） -->
      <button class="btn" @click="$emit('retranslate')">{{ t('controlBar.retranslate') }}</button>

      <!-- 原文/译文切换 -->
      <button class="btn" @click="$emit('toggleOriginal')">
        {{ showOriginal ? t('controlBar.showTranslation') : t('controlBar.showOriginal') }}
      </button>

      <!-- 缓存命中提示 -->
      <span v-if="fromCache" class="cache-hint">{{ t('controlBar.cacheHit') }}</span>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

/** 翻译状态类型 */
type TranslateStatus = 'idle' | 'translating' | 'done' | 'error'

defineProps<{
  /** 翻译状态 */
  translateStatus: TranslateStatus
  /** 是否显示原文 */
  showOriginal: boolean
  /** 是否有翻译结果 */
  hasTranslation: boolean
  /** 错误信息 */
  errorMessage?: string
  /** 是否来自历史缓存 */
  fromCache?: boolean
  /** OCR 是否正在识别中（idle 状态下"复制原文"按钮的加载状态） */
  ocrLoading?: boolean
}>()

defineEmits<{
  /** 翻译按钮点击 */
  translate: []
  /** 重新翻译按钮点击（强制调用API，跳过缓存） */
  retranslate: []
  /** 复制原文按钮点击 */
  copyOriginal: []
  /** 复制译文按钮点击 */
  copyTranslation: []
  /** 原文/译文切换按钮点击 */
  toggleOriginal: []
  /** idle 状态下"复制原文"按钮点击（需先执行 OCR） */
  ocrCopyOriginal: []
}>()
</script>

<style scoped>
.control-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
  background: transparent;
  min-height: var(--control-bar-height);
}

.btn {
  padding: 4px 12px;
  border: none;
  border-radius: var(--border-radius);
  background: #3a3a40;
  color: #f0f0f0;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

.btn:hover {
  background: #4e4e55;
}

/* 缓存命中提示 */
.cache-hint {
  font-size: 12px;
  color: #4caf50;
  margin-left: auto;
  white-space: nowrap;
}

/* 错误提示信息 */
.error-msg {
  font-size: 12px;
  color: var(--color-danger);
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
