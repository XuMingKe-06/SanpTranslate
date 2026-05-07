export default {
  // Common
  common: {
    confirm: 'Confirm',
    cancel: 'Cancel',
    delete: 'Delete',
    copy: 'Copy',
    close: 'Close',
    save: 'Save',
    loading: 'Loading...',
    success: 'Success',
    error: 'Error',
  },

  // Control bar
  controlBar: {
    translate: 'AI Translate',
    retranslate: 'Retry',
    translating: 'Translating...',
    copyAll: 'Copy All',
    showOriginal: 'Show Original',
    showTranslation: 'Show Translation',
  },

  // Overlay
  overlay: {
    title: 'SnapTranslate - Screenshot Overlay',
  },

  // Pin window
  pin: {
    title: 'SnapTranslate - Pin',
  },

  // Settings
  settings: {
    title: 'Settings',
    apiConfig: 'API Configuration',
    apiBaseUrl: 'API Base URL',
    apiKey: 'API Key',
    apiKeyPlaceholder: 'Enter API key',
    apiKeyHidden: 'Key saved',
    model: 'Model',
    translateConfig: 'Translation',
    targetLanguage: 'Target Language',
    shortcutConfig: 'Shortcuts',
    captureShortcut: 'Capture Shortcut',
    pinClipboardShortcut: 'Pin from Clipboard',
    save: 'Save',
    testConnection: 'Test Connection',
    configSaved: 'Configuration saved',
    saveFailed: 'Save failed',
    loadFailed: 'Failed to load configuration',
    fillApiUrl: 'Please enter API URL first',
    fillModel: 'Please enter model name first',
    fillApiKey: 'Please configure API key first',
    connectionSuccess: 'Connection successful',
    connectionFailed: 'Connection failed',
    // Language options
    langZhCN: 'Simplified Chinese',
    langZhTW: 'Traditional Chinese',
    langEn: 'English',
    langJa: 'Japanese',
    langKo: 'Korean',
    langFr: 'French',
    langDe: 'German',
    langEs: 'Spanish',
    langRu: 'Russian',
  },

  // History
  history: {
    title: 'History',
    clearAll: 'Clear All',
    empty: 'No translation history',
    detail: 'Translation Detail',
    original: 'Original',
    translation: 'Translation',
    copyTranslation: 'Copy Translation',
    confirmDelete: 'Are you sure you want to delete this record?',
    confirmClearAll: 'Are you sure you want to clear all history? This action cannot be undone.',
    deleted: 'Deleted',
    cleared: 'All history cleared',
    deleteFailed: 'Delete failed',
    clearFailed: 'Clear failed',
    loadFailed: 'Failed to load history',
    detailLoadFailed: 'Failed to load detail',
    copySuccess: 'Copied to clipboard',
    copyFailed: 'Copy failed',
  },

  // Tray menu
  tray: {
    captureTranslate: 'Capture & Translate',
    pinFromClipboard: 'Pin from Clipboard',
    history: 'Translation History',
    settings: 'Settings',
    quit: 'Quit',
  },

  // Error messages
  error: {
    apiKeyNotConfigured: 'API key not configured. Please set it in Settings.',
    apiKeyInvalid: 'Invalid or expired API key',
    apiKeyNoPermission: 'No permission to access the API. Check your key permissions.',
    apiUrlNotFound: 'API URL not found. Please check the URL.',
    rateLimited: 'Too many requests. Please try again later.',
    serverError: 'Server error. Please try again later.',
    noCachedScreenshot: 'No cached screenshot data. Please capture again.',
  },
}
