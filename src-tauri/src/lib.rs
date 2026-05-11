mod capture;
mod clipboard;
mod commands;
mod config;
mod error;
mod history;
mod hotkey;
mod ocr;
mod translate;
mod tray;
mod window;

use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview))
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: None }))
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .manage(Mutex::new(window::PinImageStore::default()))
        .manage(Mutex::new(window::CachedScreenStore::default()))
        .manage(Mutex::new(tray::TrayState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config,
            commands::write_clipboard_image,
            commands::read_clipboard_image,
            commands::write_clipboard_text,
            commands::close_pin_window,
            commands::get_pin_image,
            commands::capture_region_from_cache,
            commands::get_overlay_image,
            commands::store_pin_image,
            commands::translate_image,
            commands::translate_text,
            commands::get_api_key,
            commands::set_api_key,
            commands::delete_api_key,
            commands::get_config_path,
            commands::test_api_connection,
            commands::get_history_list,
            commands::get_history_detail,
            commands::delete_history,
            commands::clear_history
        ])
        .setup(|app| {
            let config_manager = config::ConfigManager::new(app.handle())?;
            let app_config = config_manager.load()?;

            // 初始化并缓存截图服务
            let capture_service = capture::CaptureService::new()
                .map_err(|e| {
                    log::error!("初始化截图服务失败: {}", e);
                    #[cfg(target_os = "macos")]
                    log::warn!(
                        "[PERMISSION] 屏幕截图需要屏幕录制权限 (macOS 10.15+)。\
                         请前往 系统设置 > 隐私与安全性 > 屏幕录制 添加终端或本应用"
                    );
                    e
                })?;
            app.manage(Mutex::new(capture_service));

            // Linux: 检测是否运行在 Wayland 会话下
            #[cfg(target_os = "linux")]
            {
                let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
                let wayland_display = std::env::var("WAYLAND_DISPLAY").unwrap_or_default();
                if session_type == "wayland" || !wayland_display.is_empty() {
                    log::warn!(
                        "[WAYLAND] 检测到 Wayland 会话，部分功能可能受限。\
                         屏幕截图需要安装 xdg-desktop-portal 和 PipeWire"
                    );
                }
            }

            // 初始化历史记录服务（SQLite 数据库）
            let data_dir = app.path().app_data_dir()
                .map_err(|e| {
                    log::error!("获取应用数据目录失败: {}", e);
                    error::AppError::ConfigError(format!("获取应用数据目录失败: {}", e))
                })?;
            let db_path = data_dir.join("data").join("history.db");
            let history_service = history::HistoryService::new(&db_path)
                .map_err(|e| {
                    log::error!("初始化历史记录服务失败: {}", e);
                    e
                })?;
            app.manage(Mutex::new(history_service));

            tray::create_tray(app.handle(), &app_config.shortcuts, &app_config.language)?;

            // 启动剪贴板图片监控（缓存最近的图片，供 Ctrl+Alt+P 跳过文本粘贴图片）
            let clipboard_cache = clipboard::ClipboardImageCache::new();
            let watcher_cache = clipboard_cache.clone();
            app.manage(clipboard_cache);
            clipboard::start_clipboard_watcher(app.handle().clone(), watcher_cache);

            #[cfg(desktop)]
            hotkey::register_hotkeys(app.handle(), &app_config.shortcuts)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
