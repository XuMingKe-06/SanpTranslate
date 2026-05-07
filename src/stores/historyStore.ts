import { defineStore } from 'pinia'
import { ref } from 'vue'
import {
  getHistoryList,
  getHistoryDetail,
  deleteHistory as deleteHistoryCmd,
  clearHistory as clearHistoryCmd,
  writeClipboardText,
  type HistoryListItem,
  type HistoryEntry,
} from '@/utils/tauri'
import { logger } from '@/utils/logger'

const TAG = 'HistoryStore'

/** 历史记录状态管理 */
export const useHistoryStore = defineStore('history', () => {
  /** 历史记录列表 */
  const historyList = ref<HistoryListItem[]>([])
  /** 是否正在加载 */
  const loading = ref(false)
  /** 当前查看的详情 */
  const currentDetail = ref<HistoryEntry | null>(null)

  /** 加载历史记录列表 */
  async function loadHistory() {
    loading.value = true
    try {
      historyList.value = await getHistoryList(50)
      logger.info(TAG, `加载历史记录成功，共 ${historyList.value.length} 条`)
    } catch (err) {
      logger.error(TAG, `加载历史记录失败: ${err}`, err)
      throw err
    } finally {
      loading.value = false
    }
  }

  /** 加载历史记录详情 */
  async function loadDetail(id: number) {
    try {
      currentDetail.value = await getHistoryDetail(id)
      logger.info(TAG, `加载历史详情成功，id=${id}`)
    } catch (err) {
      logger.error(TAG, `加载历史详情失败: id=${id}, error=${err}`, err)
      throw err
    }
  }

  /** 删除指定历史记录 */
  async function deleteHistory(id: number) {
    try {
      await deleteHistoryCmd(id)
      // 从列表中移除已删除的条目
      historyList.value = historyList.value.filter(item => item.id !== id)
      // 如果当前详情是被删除的条目，清空详情
      if (currentDetail.value?.id === id) {
        currentDetail.value = null
      }
      logger.info(TAG, `删除历史记录成功，id=${id}`)
    } catch (err) {
      logger.error(TAG, `删除历史记录失败: id=${id}, error=${err}`, err)
      throw err
    }
  }

  /** 清空所有历史记录 */
  async function clearHistory() {
    try {
      await clearHistoryCmd()
      historyList.value = []
      currentDetail.value = null
      logger.info(TAG, '清空所有历史记录成功')
    } catch (err) {
      logger.error(TAG, `清空历史记录失败: ${err}`, err)
      throw err
    }
  }

  /** 复制翻译文本到剪贴板 */
  async function copyTranslation(text: string) {
    try {
      await writeClipboardText(text)
      logger.info(TAG, '翻译文本已复制到剪贴板')
    } catch (err) {
      logger.error(TAG, `复制翻译文本失败: ${err}`, err)
      throw err
    }
  }

  return {
    historyList,
    loading,
    currentDetail,
    loadHistory,
    loadDetail,
    deleteHistory,
    clearHistory,
    copyTranslation,
  }
})
