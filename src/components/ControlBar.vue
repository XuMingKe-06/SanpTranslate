<template>
  <div class="control-bar">
    <!-- idle 或 error 状态：显示 AI 翻译主按钮 -->
    <button
      v-if="translateStatus === 'idle' || translateStatus === 'error'"
      class="btn btn-primary"
      @click="$emit('translate')"
    >
      {{ translateStatus === 'error' ? t('controlBar.retranslate') : t('controlBar.translate') }}
    </button>

    <!-- error 状态：显示错误提示信息 -->
    <span v-if="translateStatus === 'error' && errorMessage" class="error-msg">
      {{ errorMessage }}
    </span>

    <!-- translating 状态：显示禁用的翻译中按钮 -->
    <button
      v-else-if="translateStatus === 'translating'"
      class="btn btn-primary"
      disabled
    >
      {{ t('controlBar.translating') }}
    </button>

    <!-- done 状态：显示操作按钮组 -->
    <template v-else-if="translateStatus === 'done'">
      <!-- 复制全部 -->
      <button class="btn" @click="$emit('copyAll')">{{ t('controlBar.copyAll') }}</button>

      <!-- 原文/译文切换 -->
      <button class="btn" @click="$emit('toggleOriginal')">
        {{ showOriginal ? t('controlBar.showTranslation') : t('controlBar.showOriginal') }}
      </button>
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
}>()

defineEmits<{
  /** 翻译按钮点击 */
  translate: []
  /** 复制全部按钮点击 */
  copyAll: []
  /** 原文/译文切换按钮点击 */
  toggleOriginal: []
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
  background: rgba(255, 255, 255, 0.15);
  color: var(--color-text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

.btn:hover {
  background: rgba(255, 255, 255, 0.25);
}

.btn-primary {
  background: var(--color-accent);
  color: #000;
}

.btn-primary:hover {
  background: var(--color-accent-hover);
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
