<template>
  <div class="history-item" @click="$emit('detail', entry)">
    <!-- 缩略图 -->
    <div class="thumbnail-wrapper">
      <img
        v-if="thumbnailUrl"
        :src="thumbnailUrl"
        class="thumbnail"
        draggable="false"
      />
      <div v-else class="thumbnail-placeholder">
        <span>?</span>
      </div>
    </div>

    <!-- 内容区域 -->
    <div class="item-content">
      <div class="item-summary">{{ entry.summary }}</div>
      <div class="item-time">{{ entry.created_at }}</div>
    </div>

    <!-- 操作按钮 -->
    <div class="item-actions" @click.stop>
      <button class="action-btn" :title="t('common.copy')" @click="$emit('copy', entry)">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
        </svg>
      </button>
      <button class="action-btn action-btn-danger" :title="t('common.delete')" @click="$emit('delete', entry.id)">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="3 6 5 6 21 6"/>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { HistoryListItem } from '@/utils/tauri'

const { t } = useI18n()

const props = defineProps<{
  /** 历史记录条目 */
  entry: HistoryListItem
}>()

defineEmits<{
  /** 查看详情 */
  detail: [entry: HistoryListItem]
  /** 复制翻译文本 */
  copy: [entry: HistoryListItem]
  /** 删除记录 */
  delete: [id: number]
}>()

// 缩略图 URL（Base64 转 data URI）
const thumbnailUrl = computed(() => {
  if (props.entry.thumbnail) {
    return `data:image/jpeg;base64,${props.entry.thumbnail}`
  }
  return ''
})
</script>

<style scoped>
.history-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.history-item:hover {
  background: rgba(255, 255, 255, 0.06);
}

.thumbnail-wrapper {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  border-radius: 4px;
  overflow: hidden;
  background: rgba(255, 255, 255, 0.05);
  display: flex;
  align-items: center;
  justify-content: center;
}

.thumbnail {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.thumbnail-placeholder {
  color: rgba(255, 255, 255, 0.2);
  font-size: 18px;
}

.item-content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.item-summary {
  font-size: 13px;
  color: #e0e0e0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.4;
}

.item-time {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.35);
  margin-top: 4px;
}

.item-actions {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.15s;
}

.history-item:hover .item-actions {
  opacity: 1;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  transition: all 0.15s;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.15);
  color: #e0e0e0;
}

.action-btn-danger:hover {
  background: rgba(255, 80, 80, 0.2);
  color: #ff6b6b;
}
</style>
