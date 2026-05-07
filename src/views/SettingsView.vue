<template>
  <n-config-provider :theme="darkTheme">
    <div class="settings-container">
      <n-spin :show="loading">
        <n-space vertical :size="16">
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
                <n-input
                  v-model:value="formData.api_key"
                  type="password"
                  show-password-on="click"
                  :placeholder="hasApiKey ? '••••••••' : t('settings.apiKeyPlaceholder')"
                />
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
                <n-input
                  v-model:value="formData.shortcuts_capture"
                  placeholder="Ctrl+Alt+L"
                />
              </n-form-item>
              <n-form-item :label="t('settings.pinClipboardShortcut')">
                <n-input
                  v-model:value="formData.shortcuts_pin_clipboard"
                  placeholder="Ctrl+Alt+P"
                />
              </n-form-item>
            </n-form>
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
  createDiscreteApi,
} from 'naive-ui'
import { useConfigStore } from '@/stores/configStore'
import { testApiConnection, type AppConfig } from '@/utils/tauri'
import { logger } from '@/utils/logger'

const TAG = 'SettingsView'
const { t } = useI18n()

// 创建独立的 message 实例，配合深色主题（无需 NMessageProvider 包裹）
const { message } = createDiscreteApi(['message'], {
  configProviderProps: {
    theme: darkTheme,
  },
})

const configStore = useConfigStore()

// 表单数据（扁平化结构，方便 v-model 双向绑定）
const formData = reactive({
  api_base_url: '',
  api_key: '',
  model: '',
  target_language: 'zh-CN',
  shortcuts_capture: '',
  shortcuts_pin_clipboard: '',
})

// 页面状态
const loading = ref(false)
const saving = ref(false)
const testing = ref(false)

// 是否已有 API 密钥（从 keyring 读取）
const hasApiKey = computed(() => !!configStore.apiKey)

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

/** 将后端配置填充到表单 */
function populateForm(config: AppConfig) {
  formData.api_base_url = config.api_base_url
  formData.model = config.model
  formData.target_language = config.target_language
  formData.shortcuts_capture = config.shortcuts.capture
  formData.shortcuts_pin_clipboard = config.shortcuts.pin_clipboard
  // API 密钥不从 keyring 填充到表单，仅通过占位符提示已有密钥
  formData.api_key = ''
}

/** 保存配置 */
async function onSave() {
  saving.value = true
  try {
    // 构建 AppConfig 对象
    const newConfig: AppConfig = {
      api_base_url: formData.api_base_url.trim(),
      model: formData.model.trim(),
      target_language: formData.target_language,
      shortcuts: {
        capture: formData.shortcuts_capture.trim(),
        pin_clipboard: formData.shortcuts_pin_clipboard.trim(),
      },
    }

    // 保存配置到 TOML 文件
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
    const result = await testApiConnection(
      formData.api_base_url.trim(),
      apiKey,
      formData.model.trim()
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

// 页面加载时初始化配置数据
onMounted(async () => {
  loading.value = true
  try {
    // 并行加载配置和 API 密钥
    await Promise.all([
      configStore.loadConfig(),
      configStore.loadApiKey(),
    ])

    // 将加载的配置填充到表单
    if (configStore.config) {
      populateForm(configStore.config)
    }
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

/* 表单项之间增加间距 */
.settings-container :deep(.n-form-item) {
  margin-bottom: 12px;
}
</style>
