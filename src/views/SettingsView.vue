<template>
  <n-config-provider :theme="darkTheme">
    <div class="settings-container">
      <n-spin :show="loading">
        <n-space vertical :size="16">
          <!-- 界面语言设置区域 -->
          <n-card :title="t('settings.languageConfig')" size="small">
            <n-form label-placement="left" label-width="100" :show-feedback="false">
              <n-form-item :label="t('settings.language')">
                <n-select
                  v-model:value="formData.language"
                  :options="uiLanguageOptions"
                  @update:value="onLanguageChange"
                />
              </n-form-item>
            </n-form>
          </n-card>

          <!-- 通用设置区域 -->
          <n-card :title="t('settings.generalConfig')" size="small">
            <n-form label-placement="left" label-width="100" :show-feedback="false">
              <n-form-item :label="t('settings.autoStart')">
                <n-switch
                  v-model:value="autoStartEnabled"
                  :loading="autoStartLoading"
                  @update:value="onToggleAutoStart"
                />
              </n-form-item>
            </n-form>
          </n-card>

          <!-- API 配置区域 -->
          <n-card :title="t('settings.apiConfig')" size="small">
            <n-form label-placement="left" label-width="100" :show-feedback="false">
              <n-form-item :label="t('settings.apiBaseUrl')">
                <n-input
                  v-model:value="formData.api_base_url"
                  placeholder="https://api.openai.com"
                />
              </n-form-item>
              <n-form-item :label="t('settings.apiKey')">
                <n-space align="center" :size="8" style="width: 100%">
                  <n-input
                    v-model:value="formData.api_key"
                    type="password"
                    show-password-on="click"
                    :placeholder="hasApiKey ? '••••••••' : t('settings.apiKeyPlaceholder')"
                    style="flex: 1"
                  />
                  <n-button
                    v-if="hasApiKey"
                    size="small"
                    type="error"
                    @click="onDeleteApiKey"
                    :loading="deleting"
                  >
                    {{ t('settings.deleteApiKey') }}
                  </n-button>
                  <n-tooltip trigger="hover" v-if="hasApiKey">
                    <template #trigger>
                      <n-icon size="18" color="#888">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
                        </svg>
                      </n-icon>
                    </template>
                    {{ t('settings.apiKeyStoredInKeyring') }}
                  </n-tooltip>
                </n-space>
              </n-form-item>
              <n-form-item :label="t('settings.model')">
                <n-input
                  v-model:value="formData.model"
                  placeholder="gpt-4o"
                />
              </n-form-item>
            </n-form>
          </n-card>

          <!-- 翻译配置区域 -->
          <n-card :title="t('settings.translateConfig')" size="small">
            <n-form label-placement="left" label-width="100" :show-feedback="false">
              <n-form-item :label="t('settings.ocrLanguage')">
                <n-select
                  v-model:value="formData.ocr_language"
                  :options="ocrLanguageOptions"
                />
              </n-form-item>
              <n-form-item :label="t('settings.targetLanguage')">
                <n-select
                  v-model:value="formData.target_language"
                  :options="languageOptions"
                />
              </n-form-item>
            </n-form>
          </n-card>

          <!-- 快捷键配置区域 -->
          <n-card :title="t('settings.shortcutConfig')" size="small">
            <n-form label-placement="left" label-width="100" :show-feedback="false">
              <n-form-item :label="t('settings.captureShortcut')">
                <ShortcutInput
                  v-model="formData.shortcuts_capture"
                  :placeholder="t('settings.clickToSet')"
                />
              </n-form-item>
              <n-form-item :label="t('settings.pinClipboardShortcut')">
                <ShortcutInput
                  v-model="formData.shortcuts_pin_clipboard"
                  :placeholder="t('settings.clickToSet')"
                />
              </n-form-item>
              <n-form-item :label="t('settings.textTranslateShortcut')">
                <ShortcutInput
                  v-model="formData.shortcuts_text_translate"
                  :placeholder="t('settings.clickToSet')"
                />
              </n-form-item>
              <n-form-item :label="''">
                <n-button size="small" @click="onRestoreDefaults">
                  {{ t('settings.restoreDefaults') }}
                </n-button>
              </n-form-item>
            </n-form>
          </n-card>

          <!-- 配置文件路径提示 -->
          <n-card :title="t('settings.configFilePath')" size="small">
            <n-space align="center" :size="8">
              <n-text code style="font-size: 12px; word-break: break-all; flex: 1">
                {{ configPath || t('common.loading') }}
              </n-text>
            </n-space>
          </n-card>

          <!-- 操作按钮 -->
          <n-space justify="center">
            <n-button type="primary" @click="onSave" :loading="saving">
              {{ t('settings.save') }}
            </n-button>
            <n-button @click="onTestConnection" :loading="testing">
              {{ t('settings.testConnection') }}
            </n-button>
          </n-space>
        </n-space>
      </n-spin>
    </div>
  </n-config-provider>
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  darkTheme,
  NConfigProvider,
  NCard,
  NForm,
  NFormItem,
  NInput,
  NSelect,
  NButton,
  NSpace,
  NSpin,
  NText,
  NTooltip,
  NIcon,
  NSwitch,
  createDiscreteApi,
} from 'naive-ui'
import { useConfigStore } from '@/stores/configStore'
import { testApiConnection, deleteApiKey, getConfigPath, enableAutoStart, disableAutoStart, isAutoStartEnabled, type AppConfig } from '@/utils/tauri'
import { logger } from '@/utils/logger'
import ShortcutInput from '@/components/ShortcutInput.vue'

const TAG = 'SettingsView'
const { t, locale } = useI18n()

// 创建独立的 message 和 dialog 实例，配合深色主题（无需 NMessageProvider/NDialogProvider 包裹）
const { message, dialog } = createDiscreteApi(['message', 'dialog'], {
  configProviderProps: {
    theme: darkTheme,
  },
})

const configStore = useConfigStore()

// 默认快捷键值
const DEFAULT_CAPTURE_SHORTCUT = 'Ctrl+Alt+L'
const DEFAULT_PIN_CLIPBOARD_SHORTCUT = 'Ctrl+Alt+P'
const DEFAULT_TEXT_TRANSLATE_SHORTCUT = 'Ctrl+Alt+M'

// 表单数据（扁平化结构，方便 v-model 双向绑定）
const formData = reactive({
  api_base_url: '',
  api_key: '',
  model: '',
  target_language: 'zh-CN',
  language: 'auto',
  ocr_language: 'auto',
  shortcuts_capture: '',
  shortcuts_pin_clipboard: '',
  shortcuts_text_translate: '',
})

// 页面状态
const loading = ref(false)
const saving = ref(false)
const testing = ref(false)
const deleting = ref(false)
const configPath = ref('')

// 开机自启动状态
const autoStartEnabled = ref(false)
const autoStartLoading = ref(false)

// 是否已有 API 密钥（从 keyring 读取）
const hasApiKey = computed(() => !!configStore.apiKey)

// 界面语言选项列表
const uiLanguageOptions = computed(() => [
  { label: t('settings.languageAuto'), value: 'auto' },
  { label: t('settings.languageZhCN'), value: 'zh-CN' },
  { label: t('settings.languageEnUS'), value: 'en-US' },
])

// 目标语言选项列表（使用 i18n 标签，支持语言切换）
const languageOptions = computed(() => [
  { label: t('settings.langZhCN'), value: 'zh-CN' },
  { label: t('settings.langZhTW'), value: 'zh-TW' },
  { label: t('settings.langEn'), value: 'en' },
  { label: t('settings.langJa'), value: 'ja' },
  { label: t('settings.langKo'), value: 'ko' },
  { label: t('settings.langFr'), value: 'fr' },
  { label: t('settings.langDe'), value: 'de' },
  { label: t('settings.langEs'), value: 'es' },
  { label: t('settings.langRu'), value: 'ru' },
])

// OCR 识别语言选项列表
const ocrLanguageOptions = computed(() => [
  { label: t('settings.ocrLanguageAuto'), value: 'auto' },
  { label: t('settings.langZhCN'), value: 'chi_sim' },
  { label: t('settings.langEn'), value: 'eng' },
  { label: t('settings.langJa'), value: 'jpn' },
])

/** 将后端配置填充到表单 */
function populateForm(config: AppConfig) {
  formData.api_base_url = config.api_base_url
  formData.model = config.model
  formData.target_language = config.target_language
  formData.language = config.language || 'auto'
  formData.ocr_language = config.ocr_language || 'auto'
  formData.shortcuts_capture = config.shortcuts.capture
  formData.shortcuts_pin_clipboard = config.shortcuts.pin_clipboard
  formData.shortcuts_text_translate = config.shortcuts.text_translate
  // API 密钥不从 keyring 填充到表单，仅通过占位符提示已有密钥
  formData.api_key = ''
}

/** 界面语言切换时立即生效，无需点击保存 */
async function onLanguageChange(value: string) {
  // 防止并发保存
  if (configStore.loading) {
    return
  }

  // 立即更新当前窗口的语言
  if (value === 'auto') {
    const sysLang = navigator.language.startsWith('zh') ? 'zh-CN' : 'en-US'
    locale.value = sysLang
  } else {
    locale.value = value
  }

  // 保存语言设置到后端（只更新语言，不影响其他未保存的表单数据）
  const currentConfig = configStore.config
  if (currentConfig) {
    const newConfig: AppConfig = {
      ...currentConfig,
      language: value,
    }
    await configStore.updateConfig(newConfig)

    // configStore.updateConfig 内部捕获异常，需通过 error 字段判断是否成功
    if (configStore.error) {
      message.error(`${t('settings.saveFailed')}: ${configStore.error}`)
      logger.error(TAG, `保存语言设置失败: ${configStore.error}`)
    } else {
      logger.info(TAG, `界面语言即时切换并保存: config.language=${value}, locale=${locale.value}`)
    }
  }
}

/** 保存配置 */
async function onSave() {
  saving.value = true
  try {
    const newConfig: AppConfig = {
      api_base_url: formData.api_base_url.trim(),
      model: formData.model.trim(),
      target_language: formData.target_language,
      language: formData.language,
      ocr_language: formData.ocr_language,
      shortcuts: {
        capture: formData.shortcuts_capture.trim(),
        pin_clipboard: formData.shortcuts_pin_clipboard.trim(),
        text_translate: formData.shortcuts_text_translate.trim(),
      },
    }

    // 保存配置到 TOML 文件（后端会自动更新快捷键、托盘菜单和广播语言变更事件）
    await configStore.updateConfig(newConfig)

    // 如果用户输入了新的 API 密钥，保存到 keyring
    if (formData.api_key.trim()) {
      await configStore.setApiKey(formData.api_key.trim())
      formData.api_key = ''
    }

    message.success(t('settings.configSaved'))
    logger.info(TAG, '配置保存成功')
  } catch (err) {
    message.error(`${t('settings.saveFailed')}: ${err}`)
    logger.error(TAG, `配置保存失败: ${err}`)
  } finally {
    saving.value = false
  }
}

/** 恢复默认快捷键 */
function onRestoreDefaults() {
  formData.shortcuts_capture = DEFAULT_CAPTURE_SHORTCUT
  formData.shortcuts_pin_clipboard = DEFAULT_PIN_CLIPBOARD_SHORTCUT
  formData.shortcuts_text_translate = DEFAULT_TEXT_TRANSLATE_SHORTCUT
  message.info(t('settings.shortcutsRestored'))
  logger.info(TAG, '快捷键已恢复默认')
}

/** 测试 API 连接 */
async function onTestConnection() {
  if (!formData.api_base_url.trim()) {
    message.warning(t('settings.fillApiUrl'))
    return
  }
  if (!formData.model.trim()) {
    message.warning(t('settings.fillModel'))
    return
  }

  // 优先使用表单中输入的密钥，否则使用已存储的密钥
  const apiKey = formData.api_key.trim() || configStore.apiKey || ''
  if (!apiKey) {
    message.warning(t('settings.fillApiKey'))
    return
  }

  testing.value = true
  try {
    // 传入当前界面语言，使后端返回对应语言的提示信息
    const result = await testApiConnection(
      formData.api_base_url.trim(),
      apiKey,
      formData.model.trim(),
      formData.language
    )
    message.success(result)
    logger.info(TAG, 'API 连接测试成功')
  } catch (err) {
    // 后端已返回友好的错误信息，直接显示
    const errorMsg = String(err)
    message.error(errorMsg)
    logger.error(TAG, `API 连接测试失败: ${err}`)
  } finally {
    testing.value = false
  }
}

/** 删除 API 密钥 */
async function onDeleteApiKey() {
  dialog.warning({
    title: t('common.confirm'),
    content: t('settings.confirmDeleteApiKey'),
    positiveText: t('common.confirm'),
    negativeText: t('common.cancel'),
    onPositiveClick: async () => {
      deleting.value = true
      try {
        await deleteApiKey()
        // 清空 store 中的密钥
        configStore.apiKey = null
        message.success(t('settings.apiKeyDeleted'))
        logger.info(TAG, 'API 密钥已删除')
      } catch (err) {
        message.error(`${t('settings.deleteApiKeyFailed')}: ${err}`)
        logger.error(TAG, `删除 API 密钥失败: ${err}`)
      } finally {
        deleting.value = false
      }
    },
  })
}

/** 切换开机自启动 */
async function onToggleAutoStart(enabled: boolean) {
  autoStartLoading.value = true
  try {
    if (enabled) {
      await enableAutoStart()
      message.success(t('settings.autoStartEnabled'))
      logger.info(TAG, '开机自启动已开启')
    } else {
      await disableAutoStart()
      message.success(t('settings.autoStartDisabled'))
      logger.info(TAG, '开机自启动已关闭')
    }
  } catch (err) {
    // 切换失败时恢复原状态
    autoStartEnabled.value = !enabled
    message.error(`${t('settings.autoStartFailed')}: ${err}`)
    logger.error(TAG, `设置开机自启动失败: ${err}`)
  } finally {
    autoStartLoading.value = false
  }
}

// 页面加载时初始化配置数据
onMounted(async () => {
  loading.value = true
  try {
    // 并行加载配置、API 密钥、配置文件路径和开机自启动状态
    const [, , path, autoStart] = await Promise.all([
      configStore.loadConfig(),
      configStore.loadApiKey(),
      getConfigPath(),
      isAutoStartEnabled().catch(() => false),
    ])

    // 将加载的配置填充到表单
    if (configStore.config) {
      populateForm(configStore.config)
    }

    // 保存配置文件路径
    configPath.value = path

    // 保存开机自启动状态
    autoStartEnabled.value = autoStart

    logger.info(TAG, '设置页面初始化完成')
  } catch (err) {
    message.error(`${t('settings.loadFailed')}: ${err}`)
    logger.error(TAG, `加载配置失败: ${err}`)
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.settings-container {
  padding: 16px;
  height: 100vh;
  overflow-y: auto;
  box-sizing: border-box;
  background-color: #101014;
}

/* WebKit浏览器隐藏滚动条 */
.settings-container::-webkit-scrollbar {
  display: none;
}

/* 表单项之间增加间距 */
.settings-container :deep(.n-form-item) {
  margin-bottom: 12px;
}
</style>
